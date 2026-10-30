use crate::rest_api::builder::RestApiBuilder;

impl<'a> RestApiBuilder<'a> {
	/// Specifies options for the client you’re using to access REST API resources.
	/// For example, you can provide a default namespace prefix so that you don’t need to
	/// specify the prefix in your code.
	///
	/// The Call Options header can be used with sObject Basic Information, sObject Rows,
	/// sObject Rows by External ID, Query, QueryAll, and Search.
	///
	/// # Parameters
	/// * `client`: An identifier for the client sending the request. This appears in log files,
	/// helping you keep track of which client sent a request.
	/// * `namespace`: A developer namespace prefix used as the default namespace for the request.
	/// With this header field, the request resolves field names in managed packages without specified namespaces.
	///
	/// # Example
	/// when using the header:
	/// `Sforce-Call-Options: client=caseSensitiveToken; defaultNamespace=battle`
	/// Then queries such as the following succeed:
	/// `/services/data/vXX.X/query/?q=SELECT+Id+BotID__c+FROM+Account`
	/// In this case, the actual field queried is the battle__BotId__c field.
	///
	/// # See
	/// https://developer.salesforce.com/docs/platform/api-rest/guide/headers-calloptions.html
	pub fn call_options_header(&mut self, client: Option<&str>, namespace: Option<&str>) -> &mut Self {
		let value = match (client, namespace) {
			(Some(client), Some(namespace)) => {
				format!("client={}; defaultNamespace={}", client, namespace)
			}
			(Some(client), None) => {
				format!("client={}", client)
			}
			(None, Some(namespace)) => {
				format!("defaultNamespace={}", namespace)
			}
			(None, None) => return self,
		};
		self.header("Sforce-Call-Options", &value);
		self
	}
}
