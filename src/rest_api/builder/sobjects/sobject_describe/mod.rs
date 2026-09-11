use self::sobject_describe_response::SObjectDescribeResponse;
use crate::RestApi;
use crate::rest_api::builder::sobjects::SObjectsBuilder;
use anyhow::{Context, Result};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use serde_json::json;
use crate::rest_api::HTTPMethod;

mod sobject_describe_response;

impl RestApi {
	/// Get Object Metadata Using sObject Basic Information
	///
	/// Gets basic metadata for a specified object, including some object properties,
	/// recent items, and URIs for other resources related to the object.
	///
	/// # Parameters
	/// - `object_name`: The name of the Salesforce object (SObject)
	///   to sobject_describe. For example, "Account", "Contact", or a custom object like "CustomObject__c".
	///
	/// # Returns
	/// - `Result<DescribeSObjectResult, Error>`:
	///   - On success, returns a `DescribeSObjectResult` containing the metadata details
	///     of the specified Salesforce object.
	///   - On failure, returns an `Error` containing details about what went wrong.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{Credentials, RestApi, AuthUrl};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client = RestApi::new(AuthUrl::new("force://PlatformCLI::token@example.com")?).await?;
	///
	///     match client.describe_sobject("Account").await {
	///         Ok(response) => println!("Describe result: {:?}", response),
	///         Err(e) => println!("Error retrieving object description: {:?}", e),
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # Note
	/// To retrieve the complete metadata for an object, use the [describe_sobject](#method.describe_sobject) method
	///
	/// # See
	/// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_basic_info_get.htm>
	pub async fn describe_sobject(&self, object_name: &str) -> Result<SObjectDescribeResponse> {
		self.build()
			.sobjects()
			.describe_sobject(object_name)
			.send()
			.await
	}
}

impl<'a> SObjectsBuilder<'a> {

	/// Get Object Metadata Using sObject Basic Information
	///
	/// Gets basic metadata for a specified object, including some object properties,
	/// recent items, and URIs for other resources related to the object.
	///
	/// # Parameters
	/// - `object_name`: The name of the Salesforce object (SObject)
	///   to sobject_describe. For example, "Account", "Contact", or a custom object like "CustomObject__c".
	///
	/// # Returns
	/// - `Result<DescribeSObjectResult, Error>`:
	///   - On success, returns a `DescribeSObjectResult` containing the metadata details
	///     of the specified Salesforce object.
	///   - On failure, returns an `Error` containing details about what went wrong.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{Credentials, RestApi, AuthUrl};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client = RestApi::new(AuthUrl::new("force://PlatformCLI::token@example.com")?).await?;
	///
	///     match client.build()
	/// 		.call_options_header(None, Some("MyNamespace"))
	///         .sobjects()
	/// 		.describe_sobject("CustomObject__c")
	///         .send()
	/// 		.await {
	///         Ok(response) => println!("Describe result: {:?}", response),
	///         Err(e) => println!("Error retrieving object description: {:?}", e),
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # Note
	/// To retrieve the complete metadata for an object, use the [describe_sobject](#method.describe_sobject) method
	///
	/// # See
	/// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_basic_info_get.htm>
	pub fn describe_sobject(&'a self, object_name: &str) -> SObjectDescribeBuilder<'a> {
		SObjectDescribeBuilder::new(self, object_name)
	}
}

pub struct SObjectDescribeBuilder<'a> {
	builder: &'a SObjectsBuilder<'a>,
	sobject_type: String,
}

impl<'a> SObjectDescribeBuilder<'a> {
	pub(crate) fn new(builder: &'a SObjectsBuilder, sobject_type: &str) -> SObjectDescribeBuilder<'a> {
		Self { builder, sobject_type: sobject_type.to_string() }
	}

	/// Sends the request to retrieve the Salesforce SObject type sobject_describe.
	///
	/// # Returns
	///
	/// - On success, returns a `Result` containing `SObjectDescribeResponse`.
	/// - On failure, returns an `Err` containing the reason for failure.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{Credentials, RestApi, AuthUrl};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client = RestApi::new(AuthUrl::new("force://PlatformCLI::token@example.com")?).await?;
	///
	///     match client.describe_sobject("Account").await {
	///         Ok(response) => println!("Describe result: {:?}", response),
	///         Err(e) => println!("Error retrieving object description: {:?}", e),
	///     }
	///     Ok(())
	/// }
	/// ```
	pub async fn send(
		self,
	) -> Result<SObjectDescribeResponse> {
		let body = json!({});
		let http_response = self
			.builder
			.send(HTTPMethod::GET, format!("{}", self.sobject_type), &body)
			.await?;

		Ok(crate::authentication::handle_json_response(http_response).await
			.context("Failed to parse describe sobject response")?)
	}
}
