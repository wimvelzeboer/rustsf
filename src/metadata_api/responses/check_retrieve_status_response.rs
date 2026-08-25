use std::io::Seek;
use base64::prelude::*;
use s_zip::{SZipError, StreamingZipReader};
use std::io::{Cursor, Read};
use zip::ZipArchive;
use crate::Error;

/*
 <?xml version=\"1.0\" encoding=\"UTF-8\"?>
 <soapenv:Envelope xmlns:soapenv=\"http://schemas.xmlsoap.org/soap/envelope/\" xmlns=\"http://soap.sforce.com/2006/04/metadata\">
   <soapenv:Body>
     <checkRetrieveStatusResponse>
       <result>
         <done>true</done>
         <fileProperties>
           <createdById>0059K00000c70QQQAY</createdById>
           <createdByName>User User</createdByName>
           <createdDate>2026-08-24T09:23:38.614Z</createdDate>
           <fileName>unpackaged/package.xml</fileName>
           <fullName>unpackaged/package.xml</fullName>
           <id></id>
           <lastModifiedById>0059K00000c70QQQAY</lastModifiedById>
           <lastModifiedByName>User User</lastModifiedByName>
           <lastModifiedDate>2026-08-24T09:23:38.614Z</lastModifiedDate>
           <manageableState>unmanaged</manageableState>
           <type>Package</type>
         </fileProperties>
         <id>09S9K00000M2pxpUAB</id>
         <status>Succeeded</status>
         <success>true</success>
         <zipFile>UEsDBBQACAgIAPNKGF0AAAAAAAAAAAAAAAAWAAAAdW5wYWNrY.......................</zipFile>
       </result>
     </checkRetrieveStatusResponse>
   </soapenv:Body>
 </soapenv:Envelope>
*/
use crate::metadata_api::errors::XmlParseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CheckRetrieveStatusResponse {
    /// Indicates whether the retrieve() call is completed (true) or not (false). This field is available in API version 31.0 and later.
    done: bool,

    /// If an error occurs during the retrieve() call, this field contains a descriptive message about this error. This field is available in API version 31.0 and later.
    #[serde(rename = "errorMessage")]
    error_message: Option<String>,

    /// If an error occurs during the retrieve() call, this field contains the status code for this error. This field is available in API version 31.0 and later.
    // For a description of each StatusCode value, see StatusCode in the SOAP API Developer Guide.
    #[serde(rename = "errorStatusCode")]
    error_status_code: Option<String>,

    /// Contains information about the properties of each component in the .zip file, and the manifest file package.xml. One object per component is returned.
    file_properties: Vec<FileProperties>,

    /// ID of the component being retrieved.
    id: Option<String>,

    /// The status of the retrieve() call. Valid values are:
    /// - Pending
    /// - InProgress
    /// - Succeeded
    /// - Failed
    status: String,

    /// Indicates whether the retrieve() call was successful (true) or not (false). This field is available in API version 31.0 and later.
    success: bool,

    /// The zip file returned by the retrieve request. Base 64-encoded binary data.
    /// Before making an API call, client applications must encode the binary attachment data as base64.
    /// Upon receiving a response, client applications must decode the base64 data to binary. This conversion is handled for you by a SOAP client.
    zip_file: Option<Vec<u8>>,
}

impl CheckRetrieveStatusResponse {
    pub fn from_xml(xml: &str) -> Result<Self, XmlParseError> {
        let doc = roxmltree::Document::parse(xml)?;
        let body = find_element(doc.root_element(), "Body")?;
        let check_retrieve_status_response = find_element(body, "checkRetrieveStatusResponse")?;
        /*let check_retrieve_status_response = match find_element(body, "checkRetrieveStatusResponse") {
            Ok(node) => node,
            Err(first_error) => {
                match find_element(body, "fault") {
                    Ok(fault) => {
                        let result = parse_text_child(fault, "faultstring")?;
                        Err(ParseError::MissingText(result.to_owned().as_str()))?
                    },
                    Err(_) => {
                        // Not even an error response
                        Err(first_error)?
                    }
                }
            }
        };*/
        let result = find_element(check_retrieve_status_response, "result")?;

        let mut file_properties = vec![];
        for element in find_elements(result, "fileProperties")? {
            file_properties.push(FileProperties::from_xml(element)?);
        }

        Ok(Self {
            done: parse_bool_child(result, "done")?,
            error_message: None,
            error_status_code: None,
            file_properties,
            id: match parse_text_child(result, "id") {
                Ok(id) => Some(id),
                Err(_) => None,
            },
            status: parse_text_child(result, "status")?,
            success: parse_bool_child(result, "success")?,
            zip_file: Some(BASE64_STANDARD
                .decode(parse_text_child(result, "zipFile")?)
                .unwrap() // fixme
            ),
        })
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn get_files(mut self) -> Result<Vec<FileProperties>> {

       match self.zip_file {
            Some(zip_file) => {

                // 1. Wrap the in-memory ZIP bytes in a Cursor (implements Read + Seek)
                let mut cursor = Cursor::new(zip_file);

                // 2. Open the archive from the cursor
                let mut archive = ZipArchive::new(&mut cursor)?;

                for file_props in &mut self.file_properties {

                    // 3. Locate the file by name
                    let mut file = match archive.by_name(&file_props.file_name) {
                        Ok(file) => file,
                        Err(_) => continue,
                    };

                    // 4. Read the file contents into a buffer
                    let mut contents = Vec::new();
                    file.read_to_end(&mut contents).unwrap();

                    file_props.contents = Some(contents);
                }
            },
            None => return Ok(vec![]),
        }

        Ok(self.file_properties)
    }

    pub fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }
    pub fn get_status(&self) -> &str {
        &self.status
    }

    pub fn get_zip_file(&self) -> Option<&[u8]> {
        self.zip_file.as_deref()
    }

    pub fn is_success(&self) -> bool {
        self.success
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileProperties {
    #[serde(rename = "createdById")]
    created_by_id: String,

    #[serde(rename = "createdByName")]
    created_by_name: String,

    #[serde(rename = "createdDate")]
    created_date: String,

    #[serde(rename = "fileName")]
    file_name: String,

    #[serde(rename = "fullName")]
    full_name: String,

    #[serde(rename = "id")]
    id: Option<String>,

    #[serde(rename = "lastModifiedById")]
    last_modified_by_id: String,

    #[serde(rename = "lastModifiedByName")]
    last_modified_by_name: String,

    #[serde(rename = "lastModifiedDate")]
    last_modified_date: String,

    #[serde(rename = "manageableState")]
    manageable_state: String,

    #[serde(rename = "type")]
    file_type: String,

    contents: Option<Vec<u8>>,
}

impl FileProperties {
    fn from_xml(node: roxmltree::Node) -> Result<Self, XmlParseError> {
        Ok(Self {
            created_by_id: parse_text_child(node, "createdById")?,
            created_by_name: parse_text_child(node, "createdByName")?,
            created_date: parse_text_child(node, "createdDate")?,
            file_name: parse_text_child(node, "fileName")?,
            full_name: parse_text_child(node, "fullName")?,
            id: match parse_text_child(node, "id") {
                Ok(id) => Some(id),
                Err(_) => None,
            },
            last_modified_by_id: parse_text_child(node, "lastModifiedById")?,
            last_modified_by_name: parse_text_child(node, "lastModifiedByName")?,
            last_modified_date: parse_text_child(node, "lastModifiedDate")?,
            manageable_state: parse_text_child(node, "manageableState")?,
            file_type: parse_text_child(node, "type")?,
            contents: None,
        })
    }

    pub fn get_contents(&self) -> Option<&[u8]> {
        self.contents.as_deref()
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


fn extract_file_from_memory(zip_data: Vec<u8>, filename: &str) -> Option<Vec<u8>> {
    // 1. Wrap the in-memory ZIP bytes in a Cursor (implements Read + Seek)
    let mut cursor = Cursor::new(zip_data);

    // 2. Open the archive from the cursor
    let mut archive = ZipArchive::new(&mut cursor).ok()?;

    // 3. Locate the file by name
    let mut file = archive.by_name(filename).ok()?;

    // 4. Read the file contents into a buffer
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).ok()?;

    Some(contents)
}