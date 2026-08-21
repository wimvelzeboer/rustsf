//! # Module containing the DeployResponse struct
//!
//! This response is used with deployment, status checks, and cancellating a deployment

use serde::{Deserialize, Serialize};
use super::deploy_options::DeployOptions;
use super::deploy_result::DeployResult;

/// Represents the response for a deployment request.
///
/// This structure is used to deserialize or serialize deployment-related response data,
/// providing information about the deployment status, options, and results.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct DeployResponse {

    /// The ID of the deployment request
    id: String,

    /// Optional field containing the deployment options. This is serialized/deserialized
    /// with the name `"deployOptions"`.
    ///
    /// # See
    /// [DeployOptions]
    #[serde(rename = "deployOptions")]
    options: Option<DeployOptions>,

    /// The outcome or result of the deployment. This is serialized/deserialized
    ///  with the name `"deployResult"`.
    ///
    /// # See
    /// [DeployResult]
    #[serde(rename = "deployResult")]
    result: DeployResult,

    /// An optional URL reference associated with the deployment.
    url: Option<String>,

    /// An optional field representing the validated deployment request ID. This is
    /// serialized/deserialized with the name `"validatedDeployRequestId"`.
    #[serde(rename = "validatedDeployRequestId")]
    validated_deploy_request_id: Option<String>,
}

impl DeployResponse {
    /// Retrieves the deployment request ID associated with the current instance.
    ///
    /// # Returns
    /// A string slice that represents the ID of the deployment request.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, MetadataApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = MetadataApi::new(client);
    ///     let mut request = api.new_deployment_request();
    ///     request.delete_pre("ApexClass", vec!["MyClass.cls"]);
    ///
    ///     let response = api.deploy(request).await?;
    ///     println!("Deploy Id {:?}", response.get_deploy_request_id();
    ///     Ok(())
    /// }
    /// ```
    ///
    /// This function provides read-only access to the `id` field within the instance.
    pub fn get_deploy_request_id(&self) -> &str {
        &self.id
    }
}