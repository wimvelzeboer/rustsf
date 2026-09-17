use self::global_describe_response::GlobalDescribeResponse;
use crate::RestApi;
use crate::rest_api::HTTPMethod;
use crate::rest_api::builder::sobjects::SObjectsBuilder;
use anyhow::{Context, Result};
use serde_json::{json};
use chrono::Utc;

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
	/// - <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/dome_describeGlobal.htm>
	pub async fn describe_global(&self) -> Result<GlobalDescribeResponse> {
		self.build().sobjects().describe_global().send().await
	}

	/// Get a List of Objects
	///
	/// Sends a request to the Salesforce API to retrieve metadata information about
	/// global objects (SObjects) available in the Salesforce instance,
	/// which are modified since the given date
	///
	///
	/// # Parameters
	/// - `since` (String): A string representation of the timestamp
	///     (in `EEE, dd MMM yyyy HH:mm:ss z` format,  eg. `Mon, 30 Nov 2020 08:34:54 MST`).
	///
	/// # Returns
	/// - `Ok(DescribeGlobalResponse)`: On success, returns a `DescribeGlobalResponse` containing the list of global
	///   sObjects and their metadata.
	/// - `Err(Error)`: If an error occurs during the request or while processing the response, an `Error` is returned.
	///
	/// # Errors
	/// - Returns an error if:
	///     - The client's base version path is invalid.
	///     - The HTTP request fails.
	///     - The response cannot be parsed into a valid `DescribeGlobalResponse`.
	///     - If there is no available object’s, a 304 Not Modified status code is returned  // todo - it should return an empty vec..
	///
	/// # Example
	/// ```rust
	/// use rustsf::{Client, Credentials, RestApi};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     use rustsf::RestApi;
	///     let client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///
	///     let mut api = RestApi::new(client);
	///     let since = "Wed, 21 Oct 2015 07:28:00 GMT".to_string();
	///     match api.describe_global_modified(since).await {
	///         Ok(response) => println!("Retrieved modified sObjects: {:?}", response),
	///         Err(err) => println!("Failed to retrieve modified sObjects: {:?}", err),
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # See
	/// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_describeGlobal.htm>
	pub async fn describe_global_modified(&mut self, datetime: chrono::DateTime<Utc>) -> Result<GlobalDescribeResponse> {
		let since = datetime.format("%a, %d %b %Y %H:%M:%S %:z").to_string();
		Ok(self.build()
			.if_modified_since_header(&since)
			.sobjects()
			.describe_global()
			.send()
			.await?) // fixme Can potentially return 304 if nothing could be found, add something nice here
	}

	/// Get a List of Objects
	///
	/// Sends a request to the Salesforce API to retrieve metadata information about
	/// global objects (SObjects) available in the Salesforce instance,
	/// that haven’t been modified after that date and time.
	///
	///
	/// # Parameters
	/// - `since` (String): A string representation of the timestamp
	///     (in `EEE, dd MMM yyyy HH:mm:ss z` format,  eg. `Mon, 30 Nov 2020 08:34:54 MST`).
	///
	/// # Returns
	/// - `Ok(DescribeGlobalResponse)`: On success, returns a `DescribeGlobalResponse` containing the list of global
	///   sObjects and their metadata.
	/// - `Err(Error)`: If an error occurs during the request or while processing the response, an `Error` is returned.
	///
	/// # Errors
	/// - Returns an error if:
	///     - The client's base version path is invalid.
	///     - The HTTP request fails.
	///     - The response cannot be parsed into a valid `DescribeGlobalResponse`.
	///     - If there is no available object’s, a 304 Not Modified status code is returned  // todo - it should return an empty vec..
	///
	/// # Example
	/// ```rust
	/// use rustsf::{Client, Credentials, RestApi};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     use rustsf::RestApi;
	///     let client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///
	///     let mut api = RestApi::new(client);
	///     let since = "Wed, 21 Oct 2015 07:28:00 GMT".to_string();
	///     match api.describe_global_modified(since).await {
	///         Ok(response) => println!("Retrieved modified sObjects: {:?}", response),
	///         Err(err) => println!("Failed to retrieve modified sObjects: {:?}", err),
	///     }
	///     Ok(())
	/// }
	/// ```
	/// # See
	/// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_describeGlobal.htm>
	pub async fn describe_global_unmodified(&mut self, datetime: chrono::DateTime<Utc>) -> Result<GlobalDescribeResponse> {
		let since = datetime.format("%a, %d %b %Y %H:%M:%S %:z").to_string();
		Ok(self.build()
			.if_unmodified_since_header(&since)
			.sobjects()
			.describe_global()
			.send()
			.await?) // fixme Can potentially return 304 if nothing could be found, add something nice here
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
	///     match client.build()
	///         .if_modified_since_header("Wed, 21 Oct 2015 07:28:00 GMT")?
	/// 		.sobjects()
	/// 		.describe_global()
	/// 		.send()
	/// 		.await {
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
	/// To retrieve the complete metadata for an object, use the [describe_sobject](#method.describe_sobject) method.
	///
	/// Use the `if_modified_since_heaer' or 'if_unmodified_since_header' methods to filter further
	///
	/// # See
	/// - `if_modified_since_heaer`
	/// - `if_unmodified_since_header`
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

	/// Sends the request to retrieve the Salesforce SObject type sobject_basic_info.
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

		Ok(crate::client::handle_json_response(http_response)
			.await
			.context("Failed to parse describe global response")?)
	}
}
