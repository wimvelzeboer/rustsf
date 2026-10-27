use std::fmt::Debug;
use reqwest::Response;
use serde::Serialize;
use crate::rest_api::builder::sobjects::SObjectsBuilder;
use crate::errors::Error;
use crate::rest_api::HTTPMethod;
use crate::RestApi;


impl RestApi {
	/// Performs an upsert operation for a specified Salesforce object (sObject).
	///
	/// This method sends an asynchronous PATCH request to the Salesforce API to create
	/// or update a record in the specified sObject. If a record with the given key exists,
	/// it will be updated; otherwise, a new record will be created.
	///
	/// # Type Parameters
	/// * `T` - The type of the parameters being passed. It must implement the `Serialize` trait for serialization.
	///
	/// # Arguments
	/// * `sobject_name` - A `&str` representing the API name of the Salesforce sObject (e.g., "Account", "Contact").
	/// * `key_name` - A `&str` specifying the name of the external key field used to perform the upsert operation
	///   (e.g., "CustomField__c").
	/// * `key` - A `&str` providing the value of the external key. This is used to locate an existing record or determine
	///   that a new one should be created if none exists.
	/// * `params` - A serializable object (of type `T`) that contains the data to be updated or inserted for the sObject.
	///
	/// # Returns
	/// * `Ok(Response)` - A HTTP response object returned on a successful upsert operation.
	/// * `Err(Error)` - An error object in case the upsert operation fails, which encapsulates details about the failure.
	///
	/// # Errors
	/// This method returns an error in the following cases:
	/// * If the `base_path` method of the client fails to provide the base path.
	/// * If the `patch` method of the client fails to execute the HTTP request successfully.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{RestApi, AuthUrl};
	/// use serde_json::json;
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut api = RestApi::new(AuthUrl::new("force://PlatformCLI::token@example.com")?).await?;
	///
	///     let params = json!({
	///         "Name": "Example Record",
	///         "CustomField__c": "Value123"
	///     });
	///
	///     let result = api.upsert_sobject(
	///         "Account",
	///         "CustomField__c",
	///         "Value123",
	///         params
	///     ).await;
	///
	///     match result {
	///         Ok(response) => println!("Upsert successful: {:?}", response),
	///         Err(error) => eprintln!("Upsert failed: {:?}", error),
	///     }
	///     Ok(())
	/// }
	/// ```
	/// # See
	/// https://developer.salesforce.com/docs/platform/api-rest/guide/resources-sobject-upsert-patch.html
	pub async fn upsert_sobject<T: Serialize + Debug>(
		&self,
		sobject_name: &str,
		key_name: &str,
		key: &str,
		data: T,
	) -> Result<Response, Error> {
		self.build()
			.sobjects()
			.upsert_by_ext_id(sobject_name, key_name, key, data)
			.send()
			.await
	}
}


impl<'a> SObjectsBuilder<'a> {
	pub fn upsert_by_ext_id<T: Serialize + Debug>(
		&self,
		sobject_name: &str,
		ext_id_field: &str,
		ext_id: &str,
		data: T,
	) -> SObjectUpsertByExtIdBuilder<'_, T> {
		SObjectUpsertByExtIdBuilder::new(self, sobject_name, ext_id_field, ext_id, data)
	}
}

pub struct SObjectUpsertByExtIdBuilder<'a, T: Debug + Serialize> {
	builder: &'a SObjectsBuilder<'a>,
	sobject_type: String,
	ext_id_field: String,
	ext_id: String,
	data: T,
}

impl<'a, T: Debug + Serialize> SObjectUpsertByExtIdBuilder<'a, T> {
	pub(crate) fn new(
		builder: &'a SObjectsBuilder<'a>,
		sobject_name: &str,
		ext_id_field: &str,
		ext_id: &str,
		data: T,
	) -> SObjectUpsertByExtIdBuilder<'a, T> {
		Self {
			builder,
			sobject_type: sobject_name.to_string(),
			ext_id_field: ext_id_field.to_string(),
			ext_id: ext_id.to_string(),
			data,
		}
	}

	pub async fn send(&self) -> Result<Response, Error> {
		self.builder
			.send(
				HTTPMethod::PATCH,
				format!("{}/{}/{}", self.sobject_type, self.ext_id_field, self.ext_id),
				&self.data,
			)
			.await
	}
}
