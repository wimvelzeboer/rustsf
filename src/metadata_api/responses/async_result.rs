use crate::metadata_api::errors::XmlParseError;

/// # See
/// <https://developer.salesforce.com/docs/atlas.en-us.api_meta.meta/api_meta/meta_asyncresult.htm>
pub struct AsyncResult {
	/// The Async ID of the retrieve request.
	id: String,
	/*
	   /// Deprecated
	   done: bool,
	   state: String,
	*/
}

impl AsyncResult {
	pub fn from_xml(xml: &str) -> Result<Self, XmlParseError> {
		let doc = roxmltree::Document::parse(xml)?;
		let body = find_element(doc.root_element(), "Body")?;
		let retrieve_response = find_element(body, "retrieveResponse")?;
		let result = find_element(retrieve_response, "result")?;

		Ok(Self {
			// done: parse_bool_child(result, "done")?,
			id: parse_text_child(result, "id")?,
			// state: parse_text_child(result, "state")?,
		})
	}

	pub fn id(&self) -> &str {
		&self.id
	}
}

fn find_element<'a, 'input>(
	root: roxmltree::Node<'a, 'input>,
	local_name: &'static str,
) -> Result<roxmltree::Node<'a, 'input>, XmlParseError> {
	root.descendants()
		.find(|node| node.is_element() && node.tag_name().name() == local_name)
		.ok_or(XmlParseError::MissingElement(local_name))
}

fn find_elements<'a, 'input>(
	root: roxmltree::Node<'a, 'input>,
	local_name: &'static str,
) -> Result<Vec<roxmltree::Node<'a, 'input>>, XmlParseError> {
	Ok(root
		.descendants()
		.filter(|node| node.is_element() && node.tag_name().name() == local_name)
		.collect())
}

fn parse_text_child<'a, 'input>(
	root: roxmltree::Node<'a, 'input>,
	local_name: &'static str,
) -> Result<String, XmlParseError> {
	let node = find_element(root, local_name)?;
	node.text()
		.map(ToOwned::to_owned)
		.ok_or(XmlParseError::MissingText(local_name))
}

fn parse_bool_child<'a, 'input>(
	root: roxmltree::Node<'a, 'input>,
	local_name: &'static str,
) -> Result<bool, XmlParseError> {
	let value = parse_text_child(root, local_name)?;
	value.parse::<bool>().map_err(|_| XmlParseError::InvalidBool {
		element: local_name,
		value,
	})
}
