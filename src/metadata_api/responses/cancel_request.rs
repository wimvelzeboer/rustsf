//! # Module containing the structs for cancelling a deployment
//!
//!


///
/// Cancel Request Request
///
/// Required for canceling a deployment
///
/// # See
/// [cancel](crate::metadata_api::MetadataApi#method.cancel)
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CancelRequest {

    #[serde(rename = "deployResult")]
    deploy_result: CancelResult,
}

impl CancelRequest {
    /// Creates a new instance of `CancelRequest` with an initial `CancelResult` status set to "Canceling".
    ///
    /// # Returns
    ///
    /// A new `CancelRequest` instance containing a `CancelResult` initialized with a default status "Canceling".
    ///
    /// # Example
    /// ```rust
    /// let cancel_request = CancelRequest::new();
    /// assert_eq!(cancel_request.deploy_result.status, "Canceling");
    /// ```
    pub fn new() -> Self {
        CancelRequest {
            deploy_result: CancelResult {
                status: "Canceling".to_string(),
            }
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CancelResult {

    status: String,
}