//! # Salesforce Metadata API
//!
//! Metadata API enables you to deploy metadata to Salesforce,
//! this includes destructive deployments
//!
//! ## Supported Endpoints:
//! - **/services/data/vXX.X/metadata/deployRequest**, for deployments, status checks, and cancellation
//!
//! ## Supported Methods:
//! - [deploy](crate::metadata_api::MetadataApi#method.deploy), for sending deployments to the Salesforce queue
//! - [status](crate::metadata_api::MetadataApi#method.status), reports on the status of a deployment
//! - [cancel](crate::metadata_api::MetadataApi#method.cancel), cancelles a running or queued deployment
//!
//! # Notes
//! You can deploy or retrieve up to 10,000 files at once. The maximum size of the deployed or
//! retrieved .zip file is 39 MB. If the files are uncompressed in an unzipped folder,
//! the size limit is 600 MB or 629,145,600 bytes.
//! The size limit in bytes is calculated as 600 x 1024 x 1024.
//!
//! # See
//! <https://developer.salesforce.com/docs/atlas.en-us.api_meta.meta/api_meta/meta_rest_deploy.htm>
//!

use crate::metadata_api::responses::deploy_response::DeployResponse;
use crate::rest_api::responses::error_response::ErrorResponse;
use crate::{Client};
use reqwest::{Response, multipart};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use log::{debug, trace};
use crate::metadata_api::responses::cancel_request::CancelRequest;
use metadata_deployer::MetadataDeployer;
use crate::metadata_api::metadata_retriever::MetadataRetriever;
use crate::metadata_api::responses::async_result::AsyncResult;
use crate::metadata_api::responses::check_retrieve_status_response::CheckRetrieveStatusResponse;

pub mod responses;
pub mod metadata_deployer;
pub mod metadata_retriever;

const METADATA_NAMESPACE: &str = "http://soap.sforce.com/2006/04/metadata";
const ENVELOPE_NAMESPACE: &str = "http://schemas.xmlsoap.org/soap/envelope/";

pub(crate) mod errors;

/// A `MetadataApi` that represents the core component for interacting with a Metadata API.
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
    /// Creates a new instance of `MetadataApi`.
    ///
    /// # Parameters
    /// - `client`: An instance of the `Client` struct that will be used to interact with the metadata API.
    ///
    /// # Returns
    /// A new `MetadataApi` object initialized with the provided `client`.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, Credentials, MetadataApi};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client= Client::new(Credentials::new()).await?;
    ///     // Authentication logic...
    ///
    ///     let mut api = MetadataApi::new(client);
    ///
    ///     // ... add your logic here...
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(client: Client) -> Self {
        MetadataApi { client }
    }

    /// Cancels a deployment request with the specified `deploy_request_id`.
    ///
    /// # Arguments
    /// * `deploy_request_id` - The unique identifier of the deployment request to be canceled.
    ///
    /// # Returns
    /// * `Result<DeployResponse, Error>` - Returns `DeployResponse` on success, or an `Error` on a HTTP failure.
    ///
    /// # Errors
    /// This function will return an error in the following cases:
    /// * If the client's base version path retrieval fails.
    /// * If the HTTP PATCH request fails or receives an invalid response.
    /// * If the JSON response cannot be parsed correctly.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, Credentials, MetadataApi};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client= Client::new(Credentials::new()).await?;
    ///     // Authentication logic...
    ///
    ///     let mut api = MetadataApi::new(client);
    ///
    ///     let mut request = api.new_deployment_request();
    ///     // ... add your logic here to add things to the deployment
    ///     let response = api.deploy(request).await?;
    ///
    ///     // Cancel the deployment
    ///     let response = api.cancel(response.get_deploy_request_id()).await?;
    ///     println!("Cancel response {:?}", response);
    ///     Ok(())
    /// }
    /// ```
    pub async fn cancel(&mut self, deploy_request_id: &str) -> Result<DeployResponse> {
        let response = self
            .client
            .patch(
                // "http://localhost:3000".to_string(),
                format!(
                    "{}/metadata/deployRequest/{}",
                    self.client.base_version_path()?,
                    deploy_request_id,
                ),
                CancelRequest::new(),
            )
            .await?;
        handle_json_response(response).await
    }

    /// Asynchronously deploys metadata to the server using the given `MetadataRequest`.
    ///
    /// # Parameters
    /// - `request`: The `MetadataRequest` containing the deployment options and the zipped files to be deployed.
    ///
    /// # Returns
    /// This function returns a `Result` containing:
    /// - `DeployResponse` if the HTTP request was successful, it contains the response body.
    /// - `Error` if any error occurs during the HTTP request process.
    ///
    /// # Examples
    /// ```rust
    /// use rustsf::{Client, Credentials, MetadataApi};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client= Client::new(Credentials::new()).await?;
    ///     // Authentication logic...
    ///
    ///     let mut api = MetadataApi::new(client);
    ///     let mut request = api.new_deployment_request();
    ///     // ... add your logic here to add things to the deployment
    ///     let response = api.deploy(request).await?;
    ///     println!("Deployment response {:?}", response);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    /// - The deployed zip file must be named `deploy.zip`.
    /// - Make sure the server's base version path is correctly configured in the `self.client`.
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_meta.meta/api_meta/meta_rest_deploy.htm>
    pub async fn deploy(&mut self, request: MetadataDeployer) -> Result<DeployResponse> {
        // create the form
        let form = multipart::Form::new()
            .part(
                "json",
                multipart::Part::bytes(request.get_deploy_request_json()?.into_bytes())
                    .mime_str("application/json")?,
            )
            .part(
                "zipfile",
                multipart::Part::bytes(request.get_zip_file()?)
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
        handle_json_response(response).await
    }

    /// Creates a new `MetadataRequest` instance using the current client's configuration
    /// This instance is used to build and execute metadata deployment requests.
    ///
    /// # Returns
    /// A `MetadataRequest` object initialized with the underlying client's settings.
    ///
    /// # Examples
    /// ```rust
    /// use rustsf::{Client, Credentials, MetadataApi};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client= Client::new(Credentials::new()).await?;
    ///     // Authentication logic...
    ///
    ///     let mut api = MetadataApi::new(client);
    ///
    ///     let mut request = api.new_deployment_request();
    ///     // ... add your logic here to all things to the deployment
    ///     // e.g. request.add(....)?;
    ///     let response = api.deploy(deployer).await?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_meta.meta/api_meta/meta_rest_deploy.htm>
    pub fn new_deployment_request(&self) -> MetadataDeployer {
        MetadataDeployer::new(&self.client)
    }

    pub fn new_retrieval_request(&self) -> MetadataRetriever {
        MetadataRetriever::new()
    }

    pub async fn retrieve(&mut self, request: MetadataRetriever) -> Result<AsyncResult> {

        let body = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<soapenv:Envelope xmlns:soapenv="{}" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns="{}">
  <soapenv:Header><SessionHeader><sessionId>{}</sessionId></SessionHeader></soapenv:Header>
  <soapenv:Body>
    <retrieve>
      <retrieveRequest>
        <apiVersion>{}</apiVersion>
        <unpackaged>{}</unpackaged>
      </retrieveRequest>
    </retrieve>
  </soapenv:Body>
</soapenv:Envelope>"#,
            ENVELOPE_NAMESPACE,
            METADATA_NAMESPACE,
            escape_xml(self.session_id()?),
            self.client.version_number()?,
            request.get_package(),
        );

        let response = self.client.post_soap("retrieve", body).await?;

        let xml = response.text().await?;
        trace!("Soap Metadata API retrieve response: {}", xml );

        Ok(AsyncResult::from_xml(&xml)?)
    }

    pub async fn retrieve_status(&mut self, id: &str) -> Result<CheckRetrieveStatusResponse> {

        let body = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<soapenv:Envelope xmlns:soapenv="{}" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns="{}">
  <soapenv:Header><SessionHeader><sessionId>{}</sessionId></SessionHeader></soapenv:Header>
  <soapenv:Body>
      <checkRetrieveStatus>
        <id>{}</id>
        <includeZip>true</includeZip>
      </checkRetrieveStatus>
  </soapenv:Body>
</soapenv:Envelope>"#,
            ENVELOPE_NAMESPACE,
            METADATA_NAMESPACE,
            escape_xml(self.session_id()?),
            id,
        );

        let response = self.client.post_soap("retrieve", body).await?;

        let xml = response.text().await?;

        trace!("Soap Metadata API retrieve status response: {}", xml );

        Ok(CheckRetrieveStatusResponse::from_xml(&xml)?)
    }

    /// Retrieves the status of a deployment request.
    ///
    /// # Parameters
    /// - `deploy_request_id`: The unique identifier of the deployment request for which the status is being queried.
    /// - `include_details`: A boolean indicating whether additional details about
    ///   the deployment request should be included in the response.
    ///
    /// # Returns
    /// - On success, returns a `DeployResponse` object containing the status and
    ///   optional details of the deployment request.
    /// - Returns an `Error` in case of a HTTP failure, such as a network issue or if the
    ///   deployment request ID is invalid.
    ///
    /// # Errors
    /// - This function propagates errors from underlying HTTP or JSON handling.
    /// - Returns an error if there are issues with the request parameters or
    ///   response deserialization.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, Credentials, MetadataApi};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client= Client::new(Credentials::new()).await?;
    ///     // Authentication logic...
    ///
    ///     let mut api = MetadataApi::new(client);
    ///
    ///     let mut request = api.new_deployment_request();
    ///     // ... add your logic here to add things to the deployment
    ///     let response = api.deploy(request).await?;
    ///
    ///     // Cancel the deployment
    ///     let response = api.status(response.get_deploy_request_id(), true).await?;
    ///     println!("Status response {:?}", response);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    /// - The `include_details` parameter, when set to `true`, appends an additional
    ///   query parameter to the HTTP request (`includeDetails=true`) to ask the
    ///   server for more detailed data.
    /// - The function makes use of the `handle_json_response` helper function to
    ///   process the JSON response returned by the server.
    ///
    /// # Dependencies
    /// This function requires the following:
    /// - `self.client`: An HTTP client that supports asynchronous operations.
    /// - `self.client.base_version_path()`: A method that provides the base path for
    ///   the API endpoint.
    /// - `handle_json_response`: An asynchronous function that processes the
    ///   JSON-formatted response from the server.
    pub async fn deploy_status(
        &mut self,
        deploy_request_id: &str,
        include_details: bool,
        // ) -> Result<Value> {
    ) -> Result<DeployResponse> {
        let mut params: Vec<(String, String)> = vec![];
        if include_details {
            params.push(("includeDetails".to_string(), "true".to_string()));
        }

        let response = self
            .client
            .get(
                format!(
                    "{}/metadata/deployRequest/{}",
                    self.client.base_version_path()?,
                    deploy_request_id
                ),
                params,
                vec![],
            )
            .await?;

        handle_json_response(response).await
    }

    fn session_id(&self) -> Result<&str> {
        self.client.access_token_value().ok_or(Error::NotLoggedIn)
    }
}

async fn handle_json_response<T: DeserializeOwned>(response: Response) -> Result<T> {
    if response.status().is_success() {
        Ok(response.json().await?)
    } else {
        let errors: Vec<ErrorResponse> = response.json().await?;
        Err(Error::ErrorResponses(errors))
    }
}

async fn handle_raw_response(response: Response) -> Result<String> {
    if response.status().is_success() {
        Ok(response.text().await?)
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

        metadata_type.push_str(
            format!(
                "<types>\n{}\n        <name>{}</name>\n    </types>\n",
                members, metadata
            )
            .as_str(),
        );
    }
    format!("{}\n{}\n{}", header, metadata_type, footer)
}

fn add_file_to_package(
    package: &mut HashMap<String, Vec<String>>,
    metadata_type: &str,
    file_name: &str,
) {
    // Add the file to the deployment package
    let package_file_name = file_name.split(".").next().unwrap().to_string(); // fixme remove unwrap
    println!("Adding package file: {}", package_file_name);
    package
        .entry(metadata_type.to_string())
        .or_insert_with(Vec::new)
        .push(package_file_name);
}

pub struct Deployment {}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests;
