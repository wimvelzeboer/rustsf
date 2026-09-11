use crate::rest_api::builder::RestApiBuilder;

impl<'a> RestApiBuilder<'a> {
	/// Sets the "Sforce-Duplicate-Rule-Header" to define duplicate rule behavior for Salesforce API requests.
	///
	/// This header determines how duplicate rules are applied when processing records using the REST API.
	///
	/// # Parameters
	/// - `allow_save`: allow the user to acknowledge the alert and save the duplicate record.
	///         Applicable if an alert is enabled for the action.
	/// - `include_record_details`: If TRUE, return all fields in the duplicate record, otherwise return only the ID.
	/// - `run_as_current_user`: If TRUE, use the current user’s sharing rules, otherwise use system sharing rules.
	///
	/// # Returns
	/// A mutable reference to `Self` to allow method chaining.
	///
	/// # See
	/// https://developer.salesforce.com/docs/platform/api-rest/guide/headers-duplicaterules.html
	pub fn duplicate_rule_header(
		&mut self,
		allow_save: bool,
		include_record_details: bool,
		run_as_current_user: bool,
	) -> &mut Self {
		let value = format!(
			"allowSave={}; includeRecordDetails={}; runAsCurrentUser={}",
			allow_save, include_record_details, run_as_current_user
		);
		self.header("Sforce-Duplicate-Rule-Header", &value);
		self
	}
}