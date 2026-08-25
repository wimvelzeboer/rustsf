//! # Services API
//!
//! Module for getting information about the Salesforce organization
//!
//! ## Supported Endpoints
//! - **/services/data/vXX.X/limits**
//! - **/services/data/vXX.X/resources**
//! - **/services/data/vXX.X/versions**
//!
//! ## Methods
//! - [**api_versions**](crate::rest_api::RestApi#method.api_versions), lists available REST API Versions
//! - [**list_limits**](crate::rest_api::RestApi#method.list_limits), lists the limits of the Salesforce environment
//! - [**list_resources**](crate::rest_api::RestApi#method.list_resources), list available REST Resources
//!
//! # See
//! <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/using_resources_getting_info_about_my_org.htm>

use super::{RestApi, handle_json_response};
use crate::rest_api::responses::limits_response::LimitsResponse;
use crate::rest_api::responses::version_response::VersionResponse;
use std::collections::HashMap;
use anyhow::Result;

impl RestApi {
    /// Lists available REST API Versions
    ///
    /// Use the Versions resource to list summary information about each REST API version currently
    /// available, including the version, label, and a link to each version's root.
    ///
    /// # Returns
    ///
    /// An asynchronous function that returns a `Result`:
    /// - `Ok(Vec<VersionResponse>)` on success, containing a vector of `VersionResponse` objects
    ///   that encapsulate the details of each available API version.
    /// - `Err(Error)` on failure, which can occur for the following reasons:
    ///   - `Error::NotLoggedIn`: If the client is not authenticated with a valid instance URL.
    ///   - Other errors that may arise from HTTP request failures or JSON deserialization.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The client instance URL (`self.client.instance_url`) is not set, which indicates that the user is not logged in.
    /// - There is an issue performing the HTTP GET request to fetch the versions.
    /// - The server response fails to deserialize into the expected format.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, RestApi};
    /// use serde::{Deserialize, Serialize};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///     let result = api.api_versions().await;
    ///     match result {
    ///         Ok(versions) => {
    ///             for version in versions {
    ///                 println!("Version: {}", version.label);
    ///             }
    ///         }
    ///         Err(error) => eprintln!("Error retrieving versions: {:?}", error),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Implementation Details
    ///
    /// 1. Verifies that the client is logged in by checking whether the `instance_url` is set.
    /// 2. Constructs the URL for retrieving API versions (`/services/data/` under the instance URL).
    /// 3. Sends an HTTP GET request to the constructed URL.
    /// 4. Processes the response using the `handle_json_response` function to deserialize and return the parsed output.
    ///
    /// # See
    /// - [VersionResponse]
    /// - <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/dome_versions.htm>
    pub async fn api_versions(&mut self) -> Result<Vec<VersionResponse>> {
        let url = self.client.base_path()?;
        let response = self.client.get(url, vec![], vec![]).await?;
        handle_json_response(response).await
    }

    /// Lists the limits of the Salesforce environment
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, RestApi};
    /// use serde::{Deserialize, Serialize};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///     match api.list_limits().await {
    ///         Ok(limits) => {
    ///             println!("Daily API Requests: Max {}, Remaining: {}",
    ///             limits.daily_api_requests.max,
    ///             limits.daily_api_requests.remaining);
    ///         },
    ///         Err(e) => println!("Error retrieving limits: {:?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/dome_limits.htm>
    pub async fn list_limits(&mut self) -> Result<LimitsResponse> {
        let url = format!("{}/limits/", self.client.base_version_path()?);
        let response = self.client.get(url, vec![], vec![]).await?;
        handle_json_response(response).await
    }

    /// List Available REST Resources
    ///
    /// # Returns
    ///
    /// A `Result` containing a `HashMap<String, String>` on success, where the keys and values
    /// represent the resource identifiers and their corresponding details, respectively.
    /// If the operation fails, an `Error` is returned.
    ///
    /// # Errors
    ///
    /// This function will return an `Error` if:
    /// - The base path or version cannot be retrieved from the client.
    /// - The HTTP GET request fails.
    /// - The response cannot be parsed as a valid JSON.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, RestApi};
    ///use std::collections::HashMap;
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///     let res: Result<HashMap<String, String>> = api.list_resources().await;
    ///     match res {
    ///         Ok(data) => println!("Resources: {:?}", data),
    ///         Err(e) => eprintln!("Failed to list resources: {}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/dome_discoveryresource.htm>
    pub async fn list_resources(&mut self) -> Result<HashMap<String, String>> {
        let url = format!("{}/{}/", self.client.base_path()?, self.client.version());
        let response = self.client.get(url, vec![], vec![]).await?;
        handle_json_response(response).await
    }
}
