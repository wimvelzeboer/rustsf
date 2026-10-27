use self::sobjects_create_request::SObjectsCreateRequest;
use super::SObjectCollectionsBuilder;
use crate::RestApi;
use crate::rest_api::HTTPMethod;
use crate::rest_api::builder::composite::sobject_collections::create::salesforce_sobjects_create_response::SalesforceSObjectsCreateResponse;
use crate::errors::Error;
use crate::rest_api::responses::rest_response::RestResponse;
use serde::Serialize;
use std::fmt::Debug;

mod salesforce_sobjects_create_response;
mod sobjects_create_request;

impl RestApi {
	/// Creates a batch of records in Salesforce using the Composite SObjects API.
	///
	/// # Type Parameters
	/// - `T`: A type that implements `Serialize` and `Debug`, representing the
	///   structure of the records to be created.
	///
	/// # Arguments
	/// - `records`: A vector of records of type `T` to be created in Salesforce.
	/// - `all_or_none`: A boolean flag that determines whether the operation should
	///   behave in an "all-or-none" manner. If `true`, the operation will fail for
	///   all records if any individual record fails; if `false`, records are
	///   processed independently.
	///
	/// # Returns
	/// - `Ok(Vec<SObjectCreateResponse>)`: A vector of `SObjectCreateResponse`
	///   objects containing the results of the creation operation for each record.
	/// - `Err(Error)`: An error if the operation fails at any stage (e.g., network
	///   error, serialization error, or API error).
	///
	/// # Errors
	/// This function will return an error in the following scenarios:
	/// - If the `base_version_path` method of the client fails.
	/// - If the HTTP `POST` request to Salesforce fails.
	/// - If there is an issue processing the JSON response from the server.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{RestApi, AuthUrl};
	/// use serde_json::json;
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let api = RestApi::new(AuthUrl::new("force://PlatformCLI::token@example.com")?).await?;
	///
	///     let mut accounts = vec![
	///         json!({"attributes": {"type": "Account"}, "Name": "Acme Inc"}),
	///         json!({"attributes": {"type": "Account"}, "Name": "Acme Co"}),
	///     ];
	///
	///     match api.create_sobjects(&mut accounts, true).await {
	///         Ok(res) => println!("Success: {:?}", res),
	///         Err(e) => println!("Failed {:?}", e),
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # See
	/// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_composite_sobjects_collections_create.htm>
	pub async fn create_sobjects<T: Serialize + Debug>(
		&self,
		records: &mut Vec<T>,
		all_or_none: bool,
	) -> Result<Vec<SalesforceSObjectsCreateResponse>, Error> {
		self
			.build()
			.composite()
			.sobject_collections()
			.create(records, all_or_none)
			.send()
			.await?
			.rest_response()
			.await
	}
}

impl<'a> SObjectCollectionsBuilder<'a> {
	/// Creates a batch of records in Salesforce using the Composite SObjects API.
	///
	/// # Type Parameters
	/// - `T`: A type that implements `Serialize` and `Debug`, representing the
	///   structure of the records to be created.
	///
	/// # Arguments
	/// - `records`: A vector of records of type `T` to be created in Salesforce.
	/// - `all_or_none`: A boolean flag that determines whether the operation should
	///   behave in an "all-or-none" manner. If `true`, the operation will fail for
	///   all records if any individual record fails; if `false`, records are
	///   processed independently.
	///
	/// # Returns
	/// - `Ok(Vec<SObjectCreateResponse>)`: A vector of `SObjectCreateResponse`
	///   objects containing the results of the creation operation for each record.
	/// - `Err(Error)`: An error if the operation fails at any stage (e.g., network
	///   error, serialization error, or API error).
	///
	/// # Errors
	/// This function will return an error in the following scenarios:
	/// - If the `base_version_path` method of the client fails.
	/// - If the HTTP `POST` request to Salesforce fails.
	/// - If there is an issue processing the JSON response from the server.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{RestApi, AuthUrl};
	/// use serde_json::json;
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let api = RestApi::new(AuthUrl::new("force://PlatformCLI::token@example.com")?).await?;
	///
	///     let mut accounts = vec![
	///         json!({"attributes": {"type": "Account"}, "Name": "Acme Inc"}),
	///         json!({"attributes": {"type": "Account"}, "Name": "Acme Co"}),
	///     ];
	///
	///     match api.create_sobjects(&mut accounts, true).await {
	///         Ok(res) => println!("Success: {:?}", res),
	///         Err(e) => println!("Failed {:?}", e),
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # See
	/// https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_composite_sobjects_collections_create.htm
	pub fn create<'builder, 'records, T: Debug + Serialize>(
		&'builder self,
		records: &'records mut Vec<T>,
		all_or_none: bool,
	) -> SObjectsCreateBuilder<'builder, 'a, 'records, T> {
		SObjectsCreateBuilder::new(self, records.as_mut_slice(), all_or_none)
	}
}

pub struct SObjectsCreateBuilder<'builder, 'api, 'records, T: Debug + Serialize> {
	builder: &'builder SObjectCollectionsBuilder<'api>,
	sobjects: &'records mut [T],
	all_or_none: bool,
}

impl<'builder, 'api, 'records, T: Debug + Serialize> SObjectsCreateBuilder<'builder, 'api, 'records, T> {
	pub(crate) fn new(
		builder: &'builder SObjectCollectionsBuilder<'api>,
		sobjects: &'records mut [T],
		all_or_none: bool,
	) -> SObjectsCreateBuilder<'builder, 'api, 'records, T> {
		SObjectsCreateBuilder {
			builder,
			sobjects,
			all_or_none,
		}
	}

	pub async fn send(&mut self) -> Result<RestResponse, Error> {

		// fixme - should we do automatically multiple requests if needed?
		if self.sobjects.len() > 200 {
			return Err(Error::Other(format!(
				"Max 200 records per request, received: {}",
				self.sobjects.len()
			)));
		}

		let request = SObjectsCreateRequest::new(self.sobjects, self.all_or_none);
		let response = self.builder.send(HTTPMethod::POST, "".to_string(), &request).await?;

		Ok(RestResponse::new(response))
	}
}
