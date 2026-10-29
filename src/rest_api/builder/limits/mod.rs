use std::collections::HashMap;
use crate::errors::Error;
use crate::rest_api::responses::limits_response::LimitsResponse;
use crate::RestApi;

impl RestApi {

	/// Lists the limits of the Salesforce environment
	///
	/// # Example
	///
	/// ```rust
	/// use rustsf::{RestApi, SalesforceCredentials};
	/// use chrono::Utc;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let credentials = SalesforceCredentials::sfdx_url("force://PlatformCLI::token@example.com").unwrap();
	///     let api = RestApi::new(credentials).await.unwrap();
	///
	///     match api.list_limits().await {
	///         Ok(limits) => {
	///             println!("Daily API Requests: Max {}, Remaining: {}",
	///             limits.daily_api_requests.max,
	///             limits.daily_api_requests.remaining);
	///         },
	///         Err(e) => println!("Error retrieving limits: {:?}", e),
	///     }
	/// }
	/// ```
	///
	/// # See
	/// https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/dome_limits.htm
	pub async fn list_limits(&self) -> Result<LimitsResponse, Error> {
		let url = format!("{}/limits/", self.client.services_data_version_path());
		let params: Vec<(String, String)> = vec![];
		let response = self.client.get_json(&url, &params, &HashMap::new()).await?;
		RestApi::handle_json_response(response).await
	}
}