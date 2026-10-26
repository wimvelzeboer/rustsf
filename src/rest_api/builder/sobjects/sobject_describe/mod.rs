use crate::rest_api::builder::sobjects::SObjectsBuilder;
use crate::rest_api::error::RestApiError;
use crate::rest_api::HTTPMethod;
use crate::RestApi;
use crate::rest_api::responses::describe_sobject_result::DescribeSObjectResult;

impl RestApi {

	/// Completely describes the individual metadata at all levels for the specified object.
	/// For example, this can be used to retrieve the fields, URLs, and child relationships for
	/// the Account object.
	///
	/// Use the builer pattern method to construct the request with the optional headers;
	/// [If-Modified-Since](crate::rest_api::builder::RestApiBuilder::if_modified_since_header) or
	/// [If-Unmodified-Since](crate::rest_api::builder::RestApiBuilder::if_unmodified_since_header)
	///
	/// # Example
	/// ```rust
	/// use rustsf::{RestApi, AuthUrl};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let client = RestApi::new(AuthUrl::new("force://PlatformCLI::token@example.com")?).await?;
	///
	///     match client.sobject_describe("Account").await {
	///         Ok(response) => println!("Describe result: {:?}", response),
	///         Err(e) => println!("Error retrieving object description: {:?}", e),
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # See
	/// - https://developer.salesforce.com/docs/platform/api-rest/guide/resources-sobject-describe.html
	/// - Builder pattern version: [describe](crate::rest_api::builder::sobjects::SObjectsBuilder::describe)
	pub async fn sobject_describe(
		&self,
		sobject_name: &str,
	) -> Result<DescribeSObjectResult, RestApiError> {
		self.build()
			.sobjects()
			.describe(sobject_name)
			.send()
			.await
	}
}

impl<'a> SObjectsBuilder<'a> {

	/// Completely describes the individual metadata at all levels for the specified object.
	/// For example, this can be used to retrieve the fields, URLs, and child relationships for
	/// the Account object.
	///
	/// For more information about the metadata that is retrieved, see DescribesObjectResult in
	/// the SOAP API Developers Guide.
	///
	/// You can use the [If-Modified-Since](crate::rest_api::builder::RestApiBuilder::if_modified_since_header)
	/// or [If-Unmodified-Since](crate::rest_api::builder::RestApiBuilder::if_unmodified_since_header)
	/// header with this resource.
	/// When using the [If-Modified-Since](crate::rest_api::builder::RestApiBuilder::if_modified_since_header)
	/// header, if no available object’s metadata has changed since the provided date,
	/// a 304 Not Modified status code is returned with no response body.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{RestApi, AuthUrl};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let client = RestApi::new(AuthUrl::new("force://PlatformCLI::token@example.com")?).await?;
	///
	///     match client.build().sobjects().describe("Account").send().await {
	///         Ok(response) => println!("Describe result: {:?}", response),
	///         Err(e) => println!("Error retrieving object description: {:?}", e),
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # See
	/// - https://developer.salesforce.com/docs/platform/api-rest/guide/resources-sobject-describe.html
	/// - Simplified version: [sobject_describe](crate::rest_api::RestApi::sobject_describe)
	pub fn describe(
		&self,
		sobject_type: &str
	) -> SObjectDescribeBuilder<'_> {
		SObjectDescribeBuilder::new(self, sobject_type)
	}
}

pub struct SObjectDescribeBuilder<'a> {
	builder: &'a SObjectsBuilder<'a>,
	sobject_type: String
}

impl<'a> SObjectDescribeBuilder<'a> {
	pub(crate) fn new(builder: &'a SObjectsBuilder, sobject_type: &str) -> SObjectDescribeBuilder<'a>
	{
		Self {
			builder,
			sobject_type: sobject_type.to_string(),
		}
	}

	pub async fn send(
		&self,
	) -> Result<DescribeSObjectResult, RestApiError> {
		let param: Vec<(&str, &str)> = vec![];
		let response = self.builder.send(
			HTTPMethod::GET,
			format!("{}/describe/", self.sobject_type),
			&param,
		).await?;
		RestApi::handle_json_response(response).await
	}
}