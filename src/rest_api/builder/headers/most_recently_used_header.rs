use crate::rest_api::builder::RestApiBuilder;

impl<'a> RestApiBuilder<'a> {

	/// Updates the "Sforce-Mru" header with a value indicating whether to update
	/// the most recently used (MRU) record.
	///
	/// # Parameters
	/// - `update_mru` (bool): A boolean flag indicating whether to update the MRU record.
	///   - `true`: Update MRU with related records if the No MRU Updates field isn’t enabled for
	/// the user on the Users page in Setup.
	///   - `false`: Disables updating of the MRU record.
	///
	/// # Returns
	/// - `&mut Self`: Returns a mutable reference to the current instance to allow method chaining.
	///
	/// # Note
	/// The MRU header can be used with these resources.
	/// - sObject Basic Information for POST requests
	/// - sObject Rows
	/// - sObject Rows by External ID
	/// - sObject Relationships
	///
	/// For integration users who make only API calls, Salesforce recommends setting the updateMru
	/// field to false to improve performance.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{RestApi, SalesforceCredentials};
	/// use chrono::Utc;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let credentials = SalesforceCredentials::sfdx_url("force://PlatformCLI::token@example.com").unwrap();
	///     let api = RestApi::new(credentials).await.unwrap();
	///
	/// 	let mut request = api.build().most_recently_used_header(true);
	/// }
	/// ```
	///
	/// In this example, the "Sforce-Mru" header will be set to "updateMru=true".
	///
	/// # See
	/// https://developer.salesforce.com/docs/platform/api-rest/guide/headers-mru.html
	pub fn most_recently_used_header(&mut self, update_mru: bool) -> &mut Self {
		let value = format!("updateMru={}",update_mru);
		self.header("Sforce-Mru", &value);
		self
	}
}