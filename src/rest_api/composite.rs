//! # Composite SObjects API
//!
//! The Composite SObjects API allows you to create multiple records in Salesforce
//! using a single HTTP request. This can be useful for bulk operations, such as
//! creating multiple accounts or contacts, up to 200 records, in a single API call.
//!
//! ## Supported Endpoints
//! - [**/services/data/vXX.X/composite/sobjects**](crate::rest_api::composite),
//!
//! ## Methods
//!
//! - [**create**](crate::rest_api::RestApi#method.create), creates a batch of records in Salesforce using the Composite SObjects API.
//! - [**sobject_by_ids**](crate::rest_api::RestApi#method.sobject_by_ids), retrieves multiple records from a Salesforce object by their IDs.
//!
//! # See
//! <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_composite_sobjects_collections.htm>
//!
//!
use std::fmt::Debug;
use serde::Serialize;
use crate::client::responses::response_error::ResponseError;
use crate::Error;
use crate::primary_types::SObject;
use crate::rest_api::{handle_json_response, RestApi};
use crate::rest_api::responses::sobject_create_response::SObjectCreateResponse;
use crate::rest_api::responses::sobject_create_request::SObjectCreateRequest;

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
    /// use rustsf::{Client, RestApi, Error, DefSObject};
    /// use rustsf::rest_api::responses::sobject_attribute::SObjectAttribute;
    ///
    /// #[DefSObject(sobject_type = "Account", fields="name")]
    /// struct NewAccount {}
    ///
    /// impl Account {
    ///     pub fn set_name(&mut self name: String) -> &mut Self {
    ///         self.name = Some(name);
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///
    ///     let accounts = vec![
    ///         Account::new().set_name("Acme Inc".to_string()),
    ///         Account::new().set_name("Acme Co".to_string()),
    ///     ];
    ///
    ///     match api.create(accounts, true).await {
    ///         Ok(res) => println!("{:?}", res),
    ///         Err(e) => println!("Failed {:?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_composite_sobjects_collections_create.htm>
    pub async fn create<T: Serialize + Debug + SObject + Clone>(
        &mut self,
        records: &mut Vec<T>,
        all_or_none: bool
    ) -> Result<Vec<SObjectCreateResponse>, Error> {
        if records.len() > 200 {
            return Err(Error::ResponseError(ResponseError::new("Max 200 records per request".to_string())));
        } else if records.len() == 0 {
            return Ok(Vec::new());
        }

        // make sure the owner id field is set
        for record in records.iter_mut() {
            if record.get_owner_id().is_none() {
                record.set_owner_id(self.client.get_user_id().as_deref().map(str::to_string).as_deref());
            }
        }

        let resource_url = format!("{}/composite/sobjects", self.client.base_version_path()?);
        let body = SObjectCreateRequest::new(records.clone(), all_or_none);
        let response = self.client.post(resource_url, body, vec![]).await?;
        let responses : Vec<SObjectCreateResponse> = handle_json_response(response).await?;

        if responses.len() != records.len() {
            return Err(
                Error::ResponseError(
                    ResponseError::new(
                    format!("Expected the same amount of responses, send {}, response {}", responses.len(), records.len()))))
        };

        for i in 0..responses.len() {
            let id = &responses.get(i).unwrap().id;
            let record = records.iter_mut().nth(i).unwrap();
            record.set_id(Some(id));
        }

        Ok(responses)
    }
       /*
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_composite_sobjects_collections_retrieve.htm>

    fixme
    pub async fn sobject_by_ids<T: DeserializeOwned>(
        &mut self,
        object_name: &str,
        ids: Vec<&str>,
        fields: Vec<&str>
    ) -> Result<Vec<T>, Error> {
        todo!();
        let url = format!("{}/composite/sobjects/{}", self.client.base_version_path()?, object_name);
        let params = vec![("fields", fields.join(",")), ("ids", ids.join(","))];
        let response = self.client.post(url, params, vec![]).await?;
        handle_json_response(response).await
    }*/
}