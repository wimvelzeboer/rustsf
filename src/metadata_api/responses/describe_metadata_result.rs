use crate::errors::XmlError;

#[derive(Debug, PartialEq, Eq)]
pub struct DescribeMetadataResult {
	metadata_objects: Vec<DescribeMetadataObject>,
	organization_namespace: Option<String>,
	partial_save_allowed: bool,
	test_required: bool,
}

impl DescribeMetadataResult {
	pub fn from_xml(xml: &str) -> Result<Self, XmlError> {
		let doc = roxmltree::Document::parse(xml)?;
		let root = doc.root_element();
		if let Some(fault) = root
			.descendants()
			.find(|node| node.is_element() && node.tag_name().name() == "Fault")
		{
			let message = find_element(fault, "faultstring")
				.ok()
				.and_then(|node| node.text())
				.map(str::trim)
				.filter(|message| !message.is_empty())
				.unwrap_or("SOAP fault returned without a faultstring");
			return Err(XmlError::SoapFault(message.to_owned()));
		}

		let body = find_element(root, "Body")?;
		let response = find_element(body, "describeMetadataResponse")?;
		let result = find_element(response, "result")?;
		let metadata_objects = result
			.children()
			.filter(|node| node.is_element() && node.tag_name().name() == "metadataObjects")
			.map(DescribeMetadataObject::from_xml)
			.collect::<Result<Vec<_>, _>>()?;

		Ok(Self {
			metadata_objects,
			organization_namespace: optional_text_child(result, "organizationNamespace")?,
			partial_save_allowed: parse_bool_child(result, "partialSaveAllowed")?,
			test_required: parse_bool_child(result, "testRequired")?,
		})
	}

	pub fn metadata_objects(&self) -> &[DescribeMetadataObject] {
		&self.metadata_objects
	}

	pub fn organization_namespace(&self) -> Option<&str> {
		self.organization_namespace.as_deref()
	}

	pub fn partial_save_allowed(&self) -> bool {
		self.partial_save_allowed
	}

	pub fn test_required(&self) -> bool {
		self.test_required
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct DescribeMetadataObject {
	child_xml_names: Vec<String>,
	directory_name: String,
	in_folder: bool,
	meta_file: bool,
	suffix: Option<String>,
	xml_name: String,
}

impl DescribeMetadataObject {
	fn from_xml(node: roxmltree::Node) -> Result<Self, XmlError> {
		let child_xml_names = node
			.children()
			.filter(|child| child.is_element() && child.tag_name().name() == "childXmlNames")
			.filter_map(|child| child.text())
			.map(str::trim)
			.filter(|name| !name.is_empty())
			.map(ToOwned::to_owned)
			.collect();

		Ok(Self {
			child_xml_names,
			directory_name: text_child(node, "directoryName")?,
			in_folder: parse_bool_child(node, "inFolder")?,
			meta_file: parse_bool_child(node, "metaFile")?,
			suffix: optional_text_child(node, "suffix")?,
			xml_name: text_child(node, "xmlName")?,
		})
	}

	pub fn child_xml_names(&self) -> &[String] {
		&self.child_xml_names
	}

	pub fn directory_name(&self) -> &str {
		&self.directory_name
	}

	pub fn in_folder(&self) -> bool {
		self.in_folder
	}

	pub fn meta_file(&self) -> bool {
		self.meta_file
	}

	pub fn suffix(&self) -> Option<&str> {
		self.suffix.as_deref()
	}

	pub fn xml_name(&self) -> &str {
		&self.xml_name
	}
}

fn find_element<'a, 'input>(
	root: roxmltree::Node<'a, 'input>,
	local_name: &'static str,
) -> Result<roxmltree::Node<'a, 'input>, XmlError> {
	root.descendants()
		.find(|node| node.is_element() && node.tag_name().name() == local_name)
		.ok_or(XmlError::MissingElement(local_name))
}

fn text_child(node: roxmltree::Node, local_name: &'static str) -> Result<String, XmlError> {
	let child = find_element(node, local_name)?;
	child
		.text()
		.map(str::trim)
		.filter(|text| !text.is_empty())
		.map(ToOwned::to_owned)
		.ok_or(XmlError::MissingText(local_name))
}

fn optional_text_child(node: roxmltree::Node, local_name: &'static str) -> Result<Option<String>, XmlError> {
	match find_element(node, local_name) {
		Ok(child) => Ok(child
			.text()
			.map(str::trim)
			.filter(|text| !text.is_empty())
			.map(ToOwned::to_owned)),
		Err(XmlError::MissingElement(_)) => Ok(None),
		Err(error) => Err(error),
	}
}

fn parse_bool_child(node: roxmltree::Node, local_name: &'static str) -> Result<bool, XmlError> {
	let value = text_child(node, local_name)?;
	value.parse::<bool>().map_err(|_| XmlError::InvalidBool {
		element: local_name,
		value,
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	const RESPONSE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns="http://soap.sforce.com/2006/04/metadata">
  <soapenv:Body>
    <describeMetadataResponse>
      <result>
        <metadataObjects>
          <childXmlNames>CustomField</childXmlNames>
          <childXmlNames>WebLink</childXmlNames>
          <directoryName>objects</directoryName>
          <inFolder>false</inFolder>
          <metaFile>true</metaFile>
          <suffix>object</suffix>
          <xmlName>CustomObject</xmlName>
        </metadataObjects>
        <metadataObjects>
          <directoryName>classes</directoryName>
          <inFolder>false</inFolder>
          <metaFile>true</metaFile>
          <suffix>cls</suffix>
          <xmlName>ApexClass</xmlName>
        </metadataObjects>
        <organizationNamespace>sample</organizationNamespace>
        <partialSaveAllowed>true</partialSaveAllowed>
        <testRequired>false</testRequired>
      </result>
    </describeMetadataResponse>
  </soapenv:Body>
</soapenv:Envelope>"#;

	#[test]
	fn parses_metadata_objects_and_optional_values() {
		let result = DescribeMetadataResult::from_xml(RESPONSE).unwrap();
		assert_eq!(result.metadata_objects().len(), 2);
		assert_eq!(
			result.metadata_objects()[0].child_xml_names(),
			["CustomField", "WebLink"]
		);
		assert_eq!(result.metadata_objects()[0].directory_name(), "objects");
		assert!(result.metadata_objects()[0].meta_file());
		assert!(result.metadata_objects()[1].child_xml_names().is_empty());
		assert_eq!(result.organization_namespace(), Some("sample"));
		assert!(result.partial_save_allowed());
		assert!(!result.test_required());
	}

	#[test]
	fn treats_empty_optional_values_as_none() {
		let xml = RESPONSE
			.replace(
				"<organizationNamespace>sample</organizationNamespace>",
				"<organizationNamespace></organizationNamespace>",
			)
			.replace("<suffix>cls</suffix>", "<suffix></suffix>");
		let result = DescribeMetadataResult::from_xml(&xml).unwrap();
		assert_eq!(result.organization_namespace(), None);
		assert_eq!(result.metadata_objects()[1].suffix(), None);
	}

	#[test]
	fn reports_soap_faults() {
		let xml = r#"<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/"><soapenv:Body><soapenv:Fault><faultstring>Insufficient access</faultstring></soapenv:Fault></soapenv:Body></soapenv:Envelope>"#;
		assert!(matches!(
			DescribeMetadataResult::from_xml(xml),
			Err(XmlError::SoapFault(message)) if message == "Insufficient access"
		));
	}

	#[test]
	fn rejects_invalid_booleans_and_malformed_xml() {
		let xml = RESPONSE.replace("<inFolder>false</inFolder>", "<inFolder>no</inFolder>");
		assert!(matches!(
			DescribeMetadataResult::from_xml(&xml),
			Err(XmlError::InvalidBool {
				element: "inFolder",
				..
			})
		));
		assert!(DescribeMetadataResult::from_xml("<not-xml").is_err());
	}
}
