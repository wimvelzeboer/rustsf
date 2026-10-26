use self::salesforce_create_response::SalesforceCreateResponse;
use self::sobject_create_result::SObjectsCreateResult;
use super::SObjectsBuilder;
use crate::RestApi;
use anyhow::Result;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use crate::rest_api::HTTPMethod;

mod salesforce_create_response;
mod sobject_create_result;

impl<'a> SObjectsBuilder<'a> {

	/// Creates a single new record in a Salesforce with the provided values.
	///
	/// # Generic
	/// - `T`: A type that implements the `Serialize` trait, representing the parameters for the new record.
	///
	/// # Parameters
	/// - `sobject_type`: A string slice that holds the API name of the Salesforce object (e.g., "Account", "Contact").
	/// - `record`: An instance of type `T` containing the details of the record to be created.
	///
	/// # Returns
	/// - `Result<T, Error>`:
	///     - On success, returns the updated instance of type 'T' containing the Salesforce record id.
	///     - On failure, returns an `Error` detailing what went wrong during the request.
	///
	/// # Errors
	/// This function returns an `Error` in the following cases:
	/// - If the Salesforce client fails to resolve the base path.
	/// - If the HTTP POST request to the Salesforce API fails.
	/// - If the response contains invalid JSON or an error from Salesforce.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{RestApi, AuthUrl};
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let client = RestApi::new(AuthUrl::new("force://PlatformCLI::token@example.com").unwrap()).await.unwrap();
	///
	///     match client.build()
	/// 			.sobjects()
	/// 			.create("Account", json!({"Name": "Acme"}))
	/// 			.send()
	/// 			.await {
	///         Ok(response) => println!("Record ID: {:?}", response.id),
	///         Err(error) => println!("Error creating account: {:?}", error),
	///     }
	/// }
	/// ```
	///
	/// # Notes
	/// - Ensure that the `sobject_type` matches the correct API name of the Salesforce object.
	/// - The `record` object should conform to the structure expected by the Salesforce API for the specified object.
	///
	/// # Dependencies
	/// - Ensure the `T` type implements the `Serialize` trait (usually through a derived implementation).
	///
	/// # See
	/// https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_basic_info_post.htm
	pub fn create<T: Debug + DeserializeOwned +  Serialize>(&'a self, sobject_type: &str, record: T) -> SObjectCreateBuilder<'a, T> {
		SObjectCreateBuilder::new(sobject_type.to_string(), record, self)
	}
}

impl RestApi {
	/// Creates a single new record in a Salesforce with the provided values.
	///
	/// # Generic
	/// - `T`: A type that implements the `Serialize` trait, representing the parameters for the new record.
	///
	/// # Parameters
	/// - `sobject_type`: A string slice that holds the API name of the Salesforce object (e.g., "Account", "Contact").
	/// - `record`: An instance of type `T` containing the details of the record to be created.
	///
	/// # Returns
	/// - `Result<T, Error>`:
	///     - On success, returns the updated instance of type 'T' containing the Salesforce record id.
	///     - On failure, returns an `Error` detailing what went wrong during the request.
	///
	/// # Errors
	/// This function returns an `Error` in the following cases:
	/// - If the Salesforce client fails to resolve the base path.
	/// - If the HTTP POST request to the Salesforce API fails.
	/// - If the response contains invalid JSON or an error from Salesforce.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{RestApi, AuthUrl};
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let client = RestApi::new(AuthUrl::new("force://PlatformCLI::token@example.com").unwrap()).await.unwrap();
	///
	///     match client.create("Account", json!({"Name": "Acme"})).await {
	///         Ok(response) => println!("Created record with ID: {}", response.id),
	///         Err(err) => eprintln!("Failed to create record: {}", err),
	///     }
	/// }
	/// ```
	///
	/// # Notes
	/// - Ensure that the `sobject_type` matches the correct API name of the Salesforce object.
	/// - The `record` object should conform to the structure expected by the Salesforce API for the specified object.
	///
	/// # Dependencies
	/// - Ensure the `T` type implements the `Serialize` trait (usually through a derived implementation).
	///
	/// # See
	/// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_basic_info_post.htm>
	pub async fn create<T: Debug + DeserializeOwned + Serialize>(
		&self,
		sobject_type: &str,
		record: T,
	) -> Result<SalesforceCreateResponse> {
		Ok(self
			.build()
			.most_recently_used_header(false)
			.sobjects()
			.create(sobject_type, record)
			.send()
			.await?
			.take_response())
	}
}

pub struct SObjectCreateBuilder<'a, T: Debug + DeserializeOwned + Serialize> {
	builder: &'a SObjectsBuilder<'a>,
	record: T,
	sobject_type: String,
}

impl<'a, T: Debug + DeserializeOwned + Serialize> SObjectCreateBuilder<'a, T> {
	pub(crate) fn new(sobject_type: String, record: T, builder: &'a SObjectsBuilder) -> SObjectCreateBuilder<'a, T>
	where
		T: Debug + Serialize,
	{
		Self {
			builder,
			record,
			sobject_type,
		}
	}

	/// Sends the creation request to the Salesforce REST API
	///
	/// This asynchronous method sends an HTTP POST request to create a new Salesforce
	/// sObject of the specified type. The `record` provided in the request payload
	/// contains the fields and values to be sent to Salesforce. The result of the
	/// request is processed, and a structured `SObjectsCreateResult` is returned.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{RestApi, AuthUrl};
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let client = RestApi::new(AuthUrl::new("force://PlatformCLI::token@example.com").unwrap()).await.unwrap();
	///
	///     match client.build()
	/// 			.sobjects()
	/// 			.create("Account", json!({"Name": "Acme"}))
	/// 			.send()
	/// 			.await {
	///         Ok(response) => println!("Record ID: {:?}", response.id),
	///         Err(error) => println!("Error creating account: {:?}", error),
	///     }
	/// }
	/// ```
	pub async fn send(self) -> Result<SObjectsCreateResult<T>> {
		let http_response = self
			.builder
			.send(HTTPMethod::POST, format!("{}", self.sobject_type), &self.record)
			.await?;

		Ok(SObjectsCreateResult::new(
			self.record,
			RestApi::handle_json_response(http_response).await?,
		))
	}
}


