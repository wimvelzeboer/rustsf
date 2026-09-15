use std::fmt::Debug;
use anyhow::{anyhow, Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::rest_api::builder::composite::sobject_collections::create::salesforce_sobjects_create_response::SalesforceSObjectsCreateResponse;
use crate::rest_api::builder::composite::sobject_collections::create::sobjects_create_response::SObjectsCreateResponse;
use super::SObjectCollectionsBuilder;
use crate::rest_api::HTTPMethod;
use crate::RestApi;
use self::sobjects_create_request::SObjectsCreateRequest;

mod sobjects_create_request;
mod sobjects_create_response;
mod salesforce_sobjects_create_response;

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
	/// use rustsf::{Client, Credentials, RestApi, DefSObject};
	/// use rustsf::rest_api::responses::sobject_attribute::SObjectAttribute;
	/// use anyhow::Result;
	///
	/// #[DefSObject(sobject_type = "Account", fields="name,owner")]
	/// struct Account {}
	///
	/// impl Account {
	///     pub fn set_name(mut self, name: String) -> Self {
	///         self.name = Some(name);
	///         self
	///     }
	/// }
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///
	///     let mut api = RestApi::new(client);
	///
	///     let mut accounts = vec![
	///         Account::new().set_name("Acme Inc".to_string()),
	///         Account::new().set_name("Acme Co".to_string()),
	///     ];
	///
	///     match api.create(&mut accounts, true).await {
	///         Ok(res) => {
	///             // Print the created accounts with their Salesforce Id
	///             println!("{:?}", accounts);
	///
	///             // Print the response from Salesforce,
	///             // including error messages if all_or_none was false and there were errors
	///             println!("{:?}", res);
	///         },
	///         Err(e) => println!("Failed {:?}", e),
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # See
	/// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_composite_sobjects_collections_create.htm>
	pub async fn create_sobjects<T: Serialize + Debug>(
		&mut self,
		records: &mut Vec<T>,
		all_or_none: bool,
	) -> Result<Vec<SalesforceSObjectsCreateResponse>> {
		Ok(self.build()
			.composite()
			.sobject_collections()
			.create(records, all_or_none)
			.send()
			.await?
			// todo - stamp id onto records?
			.take_response())
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
	/// use rustsf::{Client, Credentials, RestApi, DefSObject};
	/// use rustsf::rest_api::responses::sobject_attribute::SObjectAttribute;
	/// use anyhow::Result;
	///
	/// #[DefSObject(sobject_type = "Account", fields="name,owner")]
	/// struct Account {}
	///
	/// impl Account {
	///     pub fn set_name(mut self, name: String) -> Self {
	///         self.name = Some(name);
	///         self
	///     }
	/// }
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///
	///     let mut api = RestApi::new(client);
	///
	///     let mut accounts = vec![
	///         Account::new().set_name("Acme Inc".to_string()),
	///         Account::new().set_name("Acme Co".to_string()),
	///     ];
	///
	///     match api.create(&mut accounts, true).await {
	///         Ok(res) => {
	///             // Print the created accounts with their Salesforce Id
	///             println!("{:?}", accounts);
	///
	///             // Print the response from Salesforce,
	///             // including error messages if all_or_none was false and there were errors
	///             println!("{:?}", res);
	///         },
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

impl<'builder, 'api, 'records, T: Debug + Serialize>
SObjectsCreateBuilder<'builder, 'api, 'records, T> {
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

	pub async fn send(
		&mut self,
	) -> Result<SObjectsCreateResponse<'_, T>> {
		// fixme - should we do automatically multiple requests if needed?
		if self.sobjects.len() > 200 {
			return Err(anyhow!("Max 200 records per request"));
		} else if self.sobjects.len() == 0 {

			return Ok(SObjectsCreateResponse::new(self.sobjects, vec![]));
		}

		let request = SObjectsCreateRequest::new(self.sobjects, self.all_or_none);
		let response = self.builder.send(
			HTTPMethod::POST,
			"".to_string(),
			&request,
		).await?;

		let salesforce_response =
			crate::authentication::handle_json_response::<Vec<SalesforceSObjectsCreateResponse>>(response)
				.await
				.context("Failed to parse create records response")?;

		Ok(SObjectsCreateResponse::new(
			&mut self.sobjects,
			salesforce_response
		))

		/* todo Do we still need something of this old logic?
		if !response.status().is_success() {
			return Err(anyhow!("Failed to create records: {}", response.text().await?));
		}

		let responses: Vec<SObjectCreateResponse> = handle_json_response(response).await
			.context("Failed to parse create records response")?;
		debug!("response: {:?}", responses);

		if responses.len() != records.len() {
			return Err(anyhow!(
				"Expected the same amount of responses, send {}, response {}",
				responses.len(),
				records.len()
			));
		};

		let mut has_failures = false;
		for i in 0..responses.len() {
			let response = responses.get(i).ok_or(anyhow!("Failed to get response at index {}", i))?;
			let id = response.id();
			let record = records.iter_mut().nth(i).unwrap();
			record.set_id(id);
			if !response.is_success() {
				has_failures = true;
			}
		}

		if has_failures && all_or_none {
			return Err(anyhow!(
				"{}",
				responses
					.iter()
					.map(|r| r.to_string())
					.collect::<Vec<String>>()
					.join(", ")
			));
		}

		Ok(responses)*/
	}
}