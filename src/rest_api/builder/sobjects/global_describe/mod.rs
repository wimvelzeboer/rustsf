use self::global_describe_response::GlobalDescribeResponse;
use crate::RestApi;
use crate::rest_api::HTTPMethod;
use crate::rest_api::builder::sobjects::SObjectsBuilder;
use anyhow::{Context, Result};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::fmt::Debug;

mod global_describe_response;

impl RestApi {
	/// Get a List of Objects Describes
	///
	/// Sends a request to the Salesforce API to retrieve metadata information about all
	/// global objects (SObjects) available in the Salesforce instance.
	///
	/// This function constructs the appropriate URL using the client's base path, makes
	/// an HTTP GET request to fetch the list of global objects, and processes the JSON
	/// response to return the parsed data as a `DescribeGlobalResponse`.
	///
	/// # Errors
	///
	/// This function returns an `Error` in the following cases:
	/// - If constructing the resource URL fails.
	/// - If the HTTP GET request fails.
	/// - If the response cannot be parsed into the expected `DescribeGlobalResponse` format.
	///
	/// # Returns
	///
	/// On success, returns a `Result` wrapping a `DescribeGlobalResponse` which contains
	/// information about the global objects available in Salesforce.
	/// # Example
	/// ```rust
	/// use rustsf::{RestApi, AuthUrl};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client = RestApi::new(AuthUrl::new("force://PlatformCLI::token@example.com")?).await?;
	///
	///     match client.describe_global().await {
	///         Ok(response) => {
	///             println!("Successfully retrieved global objects: {:?}", response);
	///         }
	///         Err(e) => {
	///             eprintln!("Error retrieving global objects: {:?}", e);
	///         }
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # Note
	/// To retrieve the complete metadata for an object, use the [describe_sobject](#method.describe_sobject) method
	///
	/// # See
	/// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/dome_describeGlobal.htm>
	pub async fn describe_global(&self) -> Result<GlobalDescribeResponse> {
		self.build().sobjects().describe_global().send().await
	}
}

impl<'a> SObjectsBuilder<'a> {
	/// Get a List of Objects Describes
	///
	/// Sends a request to the Salesforce API to retrieve metadata information about all
	/// global objects (SObjects) available in the Salesforce instance.
	///
	/// This function constructs the appropriate URL using the client's base path, makes
	/// an HTTP GET request to fetch the list of global objects, and processes the JSON
	/// response to return the parsed data as a `DescribeGlobalResponse`.
	///
	/// # Errors
	///
	/// This function returns an `Error` in the following cases:
	/// - If constructing the resource URL fails.
	/// - If the HTTP GET request fails.
	/// - If the response cannot be parsed into the expected `DescribeGlobalResponse` format.
	///
	/// # Returns
	///
	/// On success, returns a `Result` wrapping a `DescribeGlobalResponse` which contains
	/// information about the global objects available in Salesforce.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{RestApi, AuthUrl};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client = RestApi::new(AuthUrl::new("force://PlatformCLI::token@example.com")?).await?;
	///
	///     match client.build().sobjects().describe_global().send().await {
	///         Ok(response) => {
	///             println!("Successfully retrieved global objects: {:?}", response);
	///         }
	///         Err(e) => {
	///             eprintln!("Error retrieving global objects: {:?}", e);
	///         }
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # Note
	/// To retrieve the complete metadata for an object, use the [describe_sobject](#method.describe_sobject) method
	///
	/// # See
	/// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/dome_describeGlobal.htm>
	pub fn describe_global(&'a self) -> GlobalDescribeBuilder<'a> {
		GlobalDescribeBuilder::new(self)
	}
}

pub struct GlobalDescribeBuilder<'a> {
	builder: &'a SObjectsBuilder<'a>,
}

impl<'a> GlobalDescribeBuilder<'a> {
	pub(crate) fn new(builder: &'a SObjectsBuilder) -> GlobalDescribeBuilder<'a> {
		Self { builder }
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
	///     match client.build().sobjects().describe_global().send().await {
	///         Ok(response) => println!("Describe result: {:?}", response),
	///         Err(e) => println!("Error retrieving object description: {:?}", e),
	///     }
	///     Ok(())
	/// }
	/// ```
	pub async fn send(self) -> Result<GlobalDescribeResponse> {
		let body = json!({});
		let url = "sobjects".to_string();
		let http_response = self
			.builder// gets the SObjectBuild
			.builder // gets the RestApiBuild
			.send(HTTPMethod::GET, url, &body)
			.await?;

		Ok(crate::authentication::handle_json_response(http_response)
			.await
			.context("Failed to parse describe global response")?)
	}
}
