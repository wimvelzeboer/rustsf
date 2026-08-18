//! # Salesforce Metadata API
//!
//! Metadata API enables you to access some entities and feature settings that you
//! can customize in the user interface.
//!

//  <https://nandblogs.com/blogs/how-to-create-and-retrieve-metadata-using-SOAP-API.html>

// Using REST API
// https://developer.salesforce.com/docs/atlas.en-us.api_meta.meta/api_meta/meta_rest_deploy.htm
// https://host/services/data/vXX.0/metadata/deployRequest

use crate::Error::RequestError;
use crate::metadata_api::responses::deploy_request::{DeployOptions, DeployRequest};
use crate::rest_api::responses::error_response::ErrorResponse;
use crate::{Client, Error};
use reqwest::multipart::Form;
use reqwest::{Response, multipart};
use s_zip::StreamingZipWriter;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io::Cursor;
use serde::Serialize;

pub mod responses;

/// Client for the Salesforce Metadata API's CRUD-based calls.
///
/// The Metadata API offers two styles: file-based calls (`deploy`/`retrieve`,
/// which take a zip and run asynchronously) and CRUD-based calls, which act on
/// components directly and return synchronously. This client covers the latter.
///
/// It exists because some operations have no equivalent elsewhere — notably
/// deleting a `CustomField`, which the Tooling API does not support at all
/// (that object exposes only Query/GET/POST/PATCH).
///
/// Unlike the REST clients this speaks SOAP, since the Metadata API has no REST
/// binding for these calls.
pub struct MetadataApi {
    client: Client,

}

impl MetadataApi {
    pub fn new(client: Client) -> Self {
        MetadataApi {
            client,
        }
    }

    pub fn new_request(&self) -> MetadataRequest {
        MetadataRequest::new(&self.client)
    }

    pub async fn deploy(&mut self, request: MetadataRequest) -> Result<Value, Error> {

        let options = request.get_deploy_request_json()?;

        let zip_file = request.get_zip_file()?;

        // create the form
        let form = multipart::Form::new()
            .part(
                "json",
                multipart::Part::bytes(options.into_bytes())
                    .mime_str("application/json")?,
            )
            .part(
                "zipfile",
                multipart::Part::bytes(zip_file)
                    .file_name("deploy.zip")
                    .mime_str("application/zip")?,
            );

        // Send request
        let response = self
            .client
            .post_multipart(
                // "http://localhost:3000".to_string(),
                format!(
                    "{}/metadata/deployRequest",
                    self.client.base_version_path()?
                ),
                vec![],
                form,
            )
            .await?;
        println!("Response: {:?}", response);
        handle_json_response(response).await

    }
}

pub struct MetadataRequest {
    version: String,
    options: DeployOptions,
    writer: StreamingZipWriter<Cursor<Vec<u8>>>,
    package: HashMap<String, Vec<String>>,
    pre_destructive: HashMap<String, Vec<String>>,
    post_destructive: HashMap<String, Vec<String>>,
}

impl MetadataRequest {
    pub fn new(client: &Client) -> Self {
        MetadataRequest {
            version: client.version_number().unwrap_or("67.0".to_string()),
            options: DeployOptions::new(),
            writer: StreamingZipWriter::from_writer(Cursor::new(Vec::new())).unwrap(),
            package: HashMap::new(),
            pre_destructive: HashMap::new(),
            post_destructive: HashMap::new(),
        }
    }

    pub fn set_options(&mut self, options: DeployOptions) {
        self.options = options;
    }

    pub fn add_metadata(
        &mut self,
        metadata_type: &str, // e.g. "PermissionSet"
        folder: &str, // e.g. "permissionsets"  todo - should should be queried from salesforce
        file_name: &str, // e.g. "IMoje_ZIP_test.permissionset-meta.xml"
        content: &[u8], //
    ) -> Result<(), Error> {

        // Add the file to the zip
        let filename = format!("{}/{}", folder, file_name);
        println!("Adding file: {}", filename);
        self.writer
            .start_entry(filename.as_str())
            .map_err(|mut e| Error::RequestError(e.to_string()))?;
        self.writer
            .write_data(content)
            .map_err(|mut e| Error::RequestError(e.to_string()))?;

        // Add the file to the deployment package
        let package_file_name = file_name.split(".").next().unwrap().to_string();
        println!("Adding package file: {}", package_file_name);
        self.package
            .entry(metadata_type.to_string())
            .or_insert_with(Vec::new)
            .push(package_file_name); // fixme remove unwrap

        Ok(())
    }

    /// Adds a package to the storage by generating its metadata and writing it to the output stream.
    ///
    /// # Parameters
    /// - `name`: A string slice representing the name of the package (e.g., the file name or package identifier).
    /// - `content`: A reference to a `HashMap` where keys are `String` values representing metadata categories or types,
    ///              and values are `Vec<String>` representing the associated data for those categories.
    ///
    /// # Returns
    /// - `Ok(())` on success, indicating that the package was successfully added.
    /// - `Err(Error)`: Returns an error if writing to the storage fails during entry creation or data writing.
    ///
    /// # Errors
    /// This method returns an `Error::RequestError` if either of the following operations fail:
    /// - `start_entry`: An error occurs while starting the entry creation process for the package.
    /// - `write_data`: An error occurs while writing the generated package metadata to the output stream.
    ///
    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    ///
    /// let mut writer = YourWriterImplementation::new();
    /// let mut manager = Manager { writer, client };
    ///
    /// let mut content = HashMap::new();
    /// content.insert("CustomField".to_string(), vec!["Account.Field__c".to_string()]);
    ///
    /// if let Err(e) = manager.add_package("package.xml", &content) {
    ///     println!("Failed to add package: {:?}", e);
    /// }
    /// ```
    fn add_package(
        &mut self,
        name: &str,
        content: HashMap<String, Vec<String>>,
    ) -> Result<(), Error> {
        let package = generate_package_xml(&self.version, &content);
        println!("Adding package: {}\n{}", name, package);

        self.writer
            .start_entry(name)
            .map_err(|mut e| Error::RequestError(e.to_string()))?;

        self.writer
            .write_data(package.as_bytes())
            .map_err(|mut e| Error::RequestError(e.to_string()))?;

        Ok(())
    }

    fn get_deploy_request_json(&self) -> Result<String, Error> {
        let request = DeployRequest {
            deploy_options: self.options.clone(),
        };
        serde_json::to_string(&request).map_err(|e| Error::RequestError(e.to_string()))
    }

    /// Register an object for deletion prior to the deployment
    pub fn delete_pre(&mut self, metadata_type: &str, objects: &[String]) -> Self {
        unimplemented!()
    }

    /// Register an object for deletion prior to the deployment
    pub fn delete_post(&mut self, metadata_type: &str, objects: &[String]) -> Self {
        unimplemented!()
    }

    /// Register an object for deployment
    pub fn add_objects(&mut self, metadata_type: &str, objects: &[String]) -> Self {
        unimplemented!()
    }

    pub fn add_object_file(&mut self, metadata_type: &str, object_file: &str) {
        unimplemented!()
    }

    fn get_zip_file(mut self) -> Result<Vec<u8>, Error> {

        // Add the package.xml files to the zip
        self.add_package("package.xml", self.package.clone())?;
        if !self.pre_destructive.is_empty() {
            self.add_package("destructiveChangesPre.xml", self.pre_destructive.clone())?;
        }
        if !self.post_destructive.is_empty() {
            self.add_package("destructiveChangesPost.xml", self.post_destructive.clone())?;
        }

        Ok(self
            .writer
            .finish()
            .map_err(|e| Error::RequestError(e.to_string()))?
            .into_inner())
    }
}

async fn handle_json_response<T: DeserializeOwned>(response: Response) -> Result<T, Error> {
    if response.status().is_success() {
        Ok(response.json().await?)
    } else {
        let errors: Vec<ErrorResponse> = response.json().await?;
        Err(Error::ErrorResponses(errors))
    }
}



fn generate_package_xml(version: &str, elements: &HashMap<String, Vec<String>>) -> String {

    let header = r#"<?xml version="1.0" encoding="UTF-8"?>
    <Package xmlns="http://soap.sforce.com/2006/04/metadata">"#;
    let footer = format!("  <version>{}</version>\n</Package>", version);

    let mut metadata_type = "".to_string();
    for (metadata, objects) in elements {
        let members = objects
            .iter()
            .map(|object| format!("        <members>{}</members>", object))
            .collect::<Vec<String>>()
            .join("\n");

        metadata_type.push_str(format!(
            "<types>\n{}\n        <name>{}</name>\n    </types>\n",
            members, metadata
        ).as_str());
    }
    format!("{}\n{}\n{}", header, metadata_type, footer)
}

pub struct Deployment {}

mod zipper;

#[cfg(test)]
mod tests;
