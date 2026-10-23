use std::collections::HashMap;
use crate::rest_api::error::RestApiError;
use crate::rest_api::responses::limits_response::LimitsResponse;
use crate::RestApi;

impl RestApi {

	/// Lists the limits of the Salesforce environment
	///
	/// # Example
	///
	/// ```rust
	/// use rustsf::{Client, Credentials, RestApi};
	/// use serde::{Deserialize, Serialize};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
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
	/// https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/dome_limits.htm
	pub async fn list_limits(&mut self) -> Result<LimitsResponse, RestApiError> {
		let url = format!("{}/limits/", self.client.services_data_version_path());
		let params: Vec<(String, String)> = vec![];
		let response = self.client.get(&url, &params, &HashMap::new()).await?;
		RestApi::handle_json_response(response).await
	}
}