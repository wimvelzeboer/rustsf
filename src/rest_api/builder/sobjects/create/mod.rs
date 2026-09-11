use self::salesforce_create_response::SalesforceCreateResponse;
use self::sobject_create_result::SObjectsCreateResult;
use super::SObjectsBuilder;
use crate::RestApi;
use crate::authentication::Client;
use anyhow::Result;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use crate::rest_api::builder::RestApiBuilder;

mod salesforce_create_response;
mod sobject_create_result;

impl<'a> SObjectsBuilder<'a> {

	pub fn create<T: Debug + DeserializeOwned +  Serialize>(&'a self, sobject_type: String, record: T) -> SObjectCreateBuilder<'a, T> {
		SObjectCreateBuilder::new(sobject_type, record, self)
	}
}

impl RestApi {
	/// Asynchronously creates a new record in Salesforce for the given SObject type and record data.
	///
	/// This function builds a request to the Salesforce API to create a new record for the specified
	/// SObject type (`sobject_type`) with the provided record data (`record`).
	///
	/// # Type Parameters
	/// * `T` - The type of the record to be created. This type must implement the `Debug`,
	///   `DeserializeOwned`, and `Serialize` traits.
	///
	/// # Arguments
	/// * `sobject_type` - The Salesforce SObject type (e.g., "Account", "Contact") of the given record.
	/// * `record` - The record data to be created in Salesforce, this type must implement the `Debug`,
	///     `DeserializeOwned`, and `Serialize` traits. of the specified type `T`.
	///
	/// # Returns
	/// A `SalesforceCreateResponse` which contains the response details from the Salesforce API, including
	/// the ID of the newly created record and any other metadata.
	///
	/// # Errors
	/// This function may return an error if the request to Salesforce fails or the response cannot be processed.
	///
	/// # Example
	/// ```rust
	/// use serde::{Serialize, Deserialize};
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let client = RestApi::new(AuthUrl::new("force://PlatformCLI::.....@....my.salesforce.com")?).await?;
	///
	///     match client.create("Account", json!({"Name": "Smith and Co 2"})).await {
	///         Ok(response) => println!("Created record with ID: {}", response.id),
	///         Err(err) => eprintln!("Failed to create record: {}", err),
	///     }
	/// }
	/// ```
	///
	/// # Notes
	/// For more complex use cases, consider using the `create` method of the `RestApiBuilder`
	/// directly. e.g. `client.build().sobjects().create("Account", json!({"Name": "Smith and Co 2"})).send().await`.
	///
	/// # See
	/// - `SalesforceCreateResponse`
	///
	pub async fn create<T: Debug + DeserializeOwned + Serialize>(
		&self,
		sobject_type: &str,
		record: T,
	) -> Result<SalesforceCreateResponse> {
		Ok(self
			.build()
			.most_recently_used_header(false)
			.sobjects()
			.create(sobject_type.to_string(), record)
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
	pub fn new(sobject_type: String, record: T, builder: &'a SObjectsBuilder) -> SObjectCreateBuilder<'a, T>
	where
		T: Debug + Serialize,
	{
		Self {
			builder,
			record,
			sobject_type,
		}
	}

	pub async fn send(self) -> Result<SObjectsCreateResult<T>> {
		let http_response = self
			.builder
			.post(format!("{}", self.sobject_type), &self.record)
			.await?;

		Ok(SObjectsCreateResult::new(
			self.record,
			crate::authentication::handle_json_response(http_response).await?,
		))
	}
}
