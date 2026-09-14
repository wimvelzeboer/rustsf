//! # SObjects API
//!
//! Module containing all the sObject Basic Information and CRUD operations on a single Salesforce object
//!
//! ## Supported Endpoints
//! - **/services/data/vXX.X/sobjects/**
//!
//! ## Methods
//! - [create_sobject](crate::rest_api::RestApi#method.create_sobject), Creates a new SObject record.
//! - [update_sobject](crate::rest_api::RestApi#method.update_sobject), Updates a single SObject record.
//! - [upsert_sobject](crate::rest_api::RestApi#method.upserts_sobject), Creates or updates a single SObject record.
//! - [delete_sobject](crate::rest_api::RestApi#method.delete_sobject), Deletes a specific SObject record by its ID.
//! - [sobject_basic_info](crate::rest_api::RestApi#method.sobject_basic_info), Basic information for a specified SObjectType.
//! - [describe_sobject](crate::rest_api::RestApi#method.describe_sobject), Detailed information about a specific SObject.
//! - [describe_global](crate::rest_api::RestApi#method.describe_global), Retrieves a list of all global SObjects.
//! - [describe_global_modified](crate::rest_api::RestApi#method.describe_global_modified), Retrieves a list of all global SObjects modified since a given date.
//! - [describe_global_unmodified](crate::rest_api::RestApi#method.describe_global_unmodified), Retrieves a list of all global SObjects not modified since a given date.

//! - [fetch_by_id](crate::rest_api::RestApi#method.fetch_by_id), Retrieves a specific SObject record by its ID.
//! - [sobject_get_deleted](crate::rest_api::RestApi#method.sobject_get_deleted), Retrieves a list of deleted SObjects.
//! - [sobject_get_updated](crate::rest_api::RestApi#method.sobject_get_updated), Retrieves a list of updated SObjects.
//!
//! # See
//! <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_basic_info.htm>

// use super::{RestApi, handle_empty_response, handle_json_response};
use super::RestApi;
use super::RestApiBuilder;
use crate::primary_types::{SObject, SObjectOwner};
use crate::rest_api::HTTPMethod;
use crate::rest_api::responses::create_response::CreateResponse;
use crate::rest_api::responses::deleted_sobjects_response::DeletedSObjectsResponse;
use crate::rest_api::responses::describe_global_response::DescribeGlobalResponse;
use crate::rest_api::responses::describe_sobject_result::DescribeSObjectResult;
use crate::rest_api::responses::sobject_attribute::SObjectAttribute;
use crate::rest_api::responses::sobject_info::SObjectInfo;
use crate::rest_api::responses::updated_sobjects_response::UpdatedSObjectsResponse;
use anyhow::Result;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

mod create;
mod delete;
mod deleted;

mod global_describe;
mod sobject_basic_info;
mod sobject_by_id;
mod sobject_describe;

mod updated;

impl<'a> RestApiBuilder<'a> {
	pub fn sobjects(&'a self) -> SObjectsBuilder<'a> {
		SObjectsBuilder::new(&self)
	}
}

pub struct SObjectsBuilder<'a> {
	builder: &'a RestApiBuilder<'a>,
}

impl<'a> SObjectsBuilder<'a> {
	pub fn new(builder: &'a RestApiBuilder) -> Self {
		SObjectsBuilder { builder }
	}

	pub(crate) async fn send_delete(
		&self,
		url: String,
	) -> Result<reqwest::Response> {
		let url = format!("sobjects/{}", url);
		self.builder.send_delete(url).await
	}

	pub(crate) async fn send<T: Debug + Serialize>(
		&self,
		method: HTTPMethod,
		url: String,
		data: &T,
	) -> Result<reqwest::Response> {
		let url = format!("sobjects/{}", url);
		self.builder.send(method, url, &data).await
	}
}

impl RestApi {
/*


	   pub async fn sobject_get_updated(
		   &mut self,
		   sobject_name: &str,
		   start_date: &str,
		   end_date: &str,
	   ) -> Result<UpdatedSObjectsResponse> {
		   let resource_url = format!(
			   "{}/sobjects/{}/updated/",
			   self.client.base_version_path()?,
			   sobject_name
		   );
		   let params = vec![
			   ("start".to_string(), start_date.to_string()),
			   ("end".to_string(), end_date.to_string()),
		   ];
		   let response = self.client.get(resource_url, params, vec![]).await?;
		   handle_json_response(response).await
	   }

	   ///
	   /// Updates an existing object in a Salesforce instance with the specified parameters.
	   ///
	   /// # Generic Parameters
	   /// - `T`: A type that implements the `Serialize` trait, representing the parameters to update the object with.
	   ///
	   /// # Arguments
	   /// - `object_name`: A string slice that specifies the name of the Salesforce object (e.g., "Account", "Contact").
	   /// - `id`: A string slice representing the unique ID of the object to be updated.
	   /// - `params`: Parameters of type `T` containing the fields and values to update on the object.
	   ///
	   /// # Returns
	   /// - `Ok(())` if the object was successfully updated.
	   /// - `Err(Error)` if an error occurs during the update process, including issues with the request or response.
	   ///
	   /// # Errors
	   /// This function returns an error if:
	   /// - The provided `object_name`, `id`, or `params` result in an invalid or malformed request.
	   /// - The Salesforce API response indicates a failure, such as an invalid ID or insufficient permissions.
	   /// - The internal HTTP client encounters an error while making the `PATCH` request.
	   ///
	   /// # Example
	   /// ```
	   /// use rustsf::{Client, Credentials, RestApi, DefSObject};
	   /// use serde::{Deserialize, Serialize};
	   /// use anyhow::Result;
	   ///
	   /// #[DefSObject(sobject_type = "Account", fields="system,type,name")]
	   /// struct Account { }
	   ///
	   /// #[tokio::main]
	   /// async fn main() -> Result<()> {
	   ///     let mut client= Client::new(Credentials::new()).await?;
	   ///     // Authentication logic...
	   ///
	   ///     let mut api = RestApi::new(client);
	   ///     let mut account = Account::new();
	   ///     account.id = Some("001D000000IqhSLIAZ".to_string());
	   ///     account.name = Some("Updated Account Name".to_string());
	   ///
	   ///     match api.update_sobject("Account", "001D000000IqhSLIAZ", account).await {
	   ///         Ok(()) => println!("Account updated successfully."),
	   ///         Err(error) => println!("Error updating account: {:?}", error),
	   ///     };
	   ///     Ok(())
	   /// }
	   /// ```
	   ///
	   /// # Notes
	   /// - This function makes an asynchronous HTTP `PATCH` request to the Salesforce API.
	   /// - It processes the response to ensure that no content (204 No Content) implies a successful update.
	   ///
	   pub async fn update_sobject<T: Serialize + Debug>(
		   &mut self,
		   object_name: &str,
		   id: &str, // Fixme get this from the object itself
		   params: T,
	   ) -> Result<()> {
		   let resource_url = format!("{}/sobjects/{}/{}", self.client.base_version_path()?, object_name, id);
		   let response = self.client.patch(resource_url, params).await?;
		   handle_empty_response(response).await
	   }

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
	   /// use rustsf::{Client, Credentials, RestApi};
	   /// use serde_json::json;
	   /// use anyhow::Result;
	   ///
	   /// #[tokio::main]
	   /// async fn main() -> Result<()> {
	   ///     let client= Client::new(Credentials::new()).await?;
	   ///     // Authentication logic...
	   ///
	   ///     let mut api = RestApi::new(client);
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
	   pub async fn upsert_sobject<T: Serialize + Debug>(
		   &mut self,
		   sobject_name: &str,
		   key_name: &str,
		   key: &str,
		   params: T,
	   ) -> Result<Response> {
		   let resource_url = format!(
			   "{}/sobjects/{}/{}/{}",
			   self.client.base_version_path()?,
			   sobject_name,
			   key_name,
			   key
		   );
		   self.client.patch(resource_url, params).await
	   }

	*/
}
