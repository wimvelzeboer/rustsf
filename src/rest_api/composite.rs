use std::fmt::Debug;
use serde::Serialize;
use crate::Error;
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
    /// struct Account {}
    ///
    /// impl Account {
    ///     pub fn new_named( name: String) -> Self {
    ///         Self {
    ///             attributes: SObjectAttribute::new("Account"),
    ///             name: Some(name),
    ///             ..Default::default()
    ///         }
    ///     }
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
    ///         Account::new_named("Acme Inc".to_string()),
    ///         Account::new_named("Acme Co".to_string()),
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
    pub async fn create<T: Serialize + Debug>(
        &mut self,
        records: Vec<T>,
        all_or_none: bool
    ) -> Result<Vec<SObjectCreateResponse>, Error> {        // todo - take a mutable reference to the records and update them with the id
        let resource_url = format!("{}/composite/sobjects", self.client.base_version_path()?);
        let body = SObjectCreateRequest::new(records, all_or_none);
        let response = self.client.post(resource_url, body, vec![]).await?;
        handle_json_response(response).await
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