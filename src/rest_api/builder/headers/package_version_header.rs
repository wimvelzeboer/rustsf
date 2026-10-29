use crate::rest_api::builder::RestApiBuilder;

impl<'a> RestApiBuilder<'a> {

	/// Sets a custom header to indicate the package version for a given namespace.
	///
	/// Specifies the version of each package referenced by a client. A package version is a number
	/// that identifies the set of components and behavior contained in a package. This header
	/// can also be used to specify a package version when making calls to an Apex REST web service.
	///
	/// The Package Version header can be used with the following resources:
	/// - Describe Global,
	/// - sObject Describe,
	/// - sObject Basic Information,
	/// - sObject Rows,
	/// - sObject Layouts,
	/// - Query,
	/// - QueryAll,
	/// - Search
	/// - sObject Rows by External ID.
	///
	/// # Parameters
	/// - `namespace`: The namespace for which the package version is being set.
	/// - `package_version`: The version of the package, e.g. "1.0".
	///
	/// # Returns
	/// - A mutable reference to the current instance (`Self`) to allow method chaining.
	///
	/// # Example
	/// ```
	/// use rustsf::{RestApi, SalesforceCredentials};
	/// use chrono::Utc;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let credentials = SalesforceCredentials::sfdx_url("force://PlatformCLI::token@example.com").unwrap();
	///     let api = RestApi::new(credentials).await.unwrap();
	///
	/// 	let mut request = api.build().package_version_header("my_namespace", "1.0.0");
	/// }
	/// ```
	///
	/// In this example, a header with the key `x-sfdc-packageversion-my_namespace` and the value `1.0.0`
	/// will be added to the `headers` collection of the client.
	///
	/// # See
	/// https://developer.salesforce.com/docs/platform/api-rest/guide/headers-packageversion.html
	pub fn package_version_header(&mut self, namespace: &str, package_version: &str) -> &mut Self {
		let key = format!("x-sfdc-packageversion-{}", namespace);
		self.headers.insert(key, package_version.to_string());
		self
	}
}