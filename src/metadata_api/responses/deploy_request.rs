//! # Module containing the structs for a deployment request
//!
//! # See
//! [deploy](crate::metadata_api::MetadataApi#method.deploy)

use super::deploy_options::DeployOptions;

/// Represents a request for deployment, encapsulating the necessary deployment options.
///
/// This struct is both serializable and deserializable using the `serde` framework, allowing it to
/// be easily converted to and from formats such as JSON.
///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DeployRequest {

    /// A `DeployOptions` structure that contains the specific parameters and configurations
    /// needed for the deployment process. This field is serialized/deserialized with the
    /// name "deployOptions".
    #[serde(rename = "deployOptions")]
    pub options: DeployOptions,
}