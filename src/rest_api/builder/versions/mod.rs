use std::collections::HashMap;
use crate::error::Error;
use crate::rest_api::HTTPMethod;
use crate::RestApi;

pub mod response;

use self::response::VersionResponse;

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
	/// use rustsf::{RestApi, AuthUrl};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let api = RestApi::new(AuthUrl::new("force://PlatformCLI::token@example.com")?).await?;
	///     match api.versions().await {
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
	/// - <https://developer.salesforce.com/docs/platform/api-rest/guide/resources-versions.html>
	pub async fn versions(&self) -> Result<Vec<VersionResponse>, Error> {
		let url = "services/data".to_string();
		let data: Vec<(String, String)> = vec![];
		let headers = HashMap::new();
		let response = self.send(HTTPMethod::GET, url, &data, &headers).await?;
		RestApi::handle_json_response(response).await
	}

}