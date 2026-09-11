use super::RestApiBuilder;

mod assignment_rule_header;
mod call_options_header;
mod conditional_request_headers;
mod duplicate_rule_header;
mod most_recently_used_header;
mod package_version_header;
mod query_options_header;

// todo do not forget to add the response headers, e.g. https://developer.salesforce.com/docs/platform/api-rest/guide/headers-api-usage.html

impl<'a> RestApiBuilder<'a> {

	/// Sets a header key-value pair in the headers map of the current object.
	///
	/// # Arguments
	///
	/// * `key` - A string slice that holds the name of the header.
	/// * `value` - A string slice that holds the value of the header.
	///
	/// # Returns
	///
	/// Returns a mutable reference to the current object, allowing method chaining.
	///
	/// # Example
	/// ```
	/// let mut request = Request::new();
	/// request.header("Sforce-Duplicate-Rule-Header", "allowSave=true")
	///        .header("Authorization", "Bearer token");
	/// ```
	pub fn header(&mut self, key: &str, value: &str) -> &mut Self {
		self.headers.insert(key.to_string(), value.to_string());
		self
	}
}
