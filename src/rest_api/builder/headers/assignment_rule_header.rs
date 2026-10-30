use crate::rest_api::builder::RestApiBuilder;

impl<'a> RestApiBuilder<'a> {
	/// The Assignment Rule header is a request header applied when creating or updating Accounts,
	/// Cases, or Leads.
	///
	/// # Parameters
	/// * auto_assign: Valid field values are ("true", "false", AssignmentRule ID).
	///     - TRUE: Active assignment rules are applied,
	///     - FALSE: Active assignment rules are not applied.
	///     - Valid AssignmentRule ID: The given AssignmentRule is applied.
	///
	/// # See
	/// https://developer.salesforce.com/docs/platform/api-rest/guide/headers-autoassign.html
	pub fn assignment_rule_header(&mut self, auto_assign: &str) -> &mut Self {
		self.header("Sforce-Auto-Assign", &auto_assign);
		self
	}
}
