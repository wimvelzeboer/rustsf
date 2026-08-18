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
//! - [describe](crate::rest_api::RestApi#method.describe), Basic information for a specified SObjectType.
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

use std::collections::HashMap;
use super::{RestApi, handle_empty_response, handle_json_response};
use crate::Error;
use crate::rest_api::responses::create_response::CreateResponse;
use crate::rest_api::responses::deleted_sobjects_response::DeletedSObjectsResponse;
use crate::rest_api::responses::describe_global_response::DescribeGlobalResponse;
use crate::rest_api::responses::describe_sobject_result::DescribeSObjectResult;
use crate::rest_api::responses::sobject_info::SObjectInfo;
use crate::rest_api::responses::updated_sobjects_response::UpdatedSObjectsResponse;
use reqwest::Response;
use serde::{Serialize};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use serde_json::Value;
use crate::primary_types::{SObject, SObjectOwner};
use crate::rest_api::responses::sobject_attribute::SObjectAttribute;

impl RestApi {
    /// Creates a single new record in a Salesforce with the provided values.
    ///
    /// # Generic
    /// - `T`: A type that implements the `Serialize` trait, representing the parameters for the new record.
    ///
    /// # Arguments
    /// - `object_name`: A string slice that holds the API name of the Salesforce object (e.g., "Account", "Contact").
    /// - `params`: An instance of type `T` containing the details of the record to be created.
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
    /// use rustsf::{Client, RestApi, Error, DefSObject};
    ///
    /// #[DefSObject(sobject_type = "Account", fields="system,type,name,owner")]
    /// struct Account { }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///     let mut account = Account::new();
    ///     account.name = Some("Example Account".to_string());
    ///
    ///     match api.create_sobject(account).await {
    ///         Ok(record) => println!("Record ID: {:?}", record.id),
    ///         Err(error) => println!("Error creating account: {:?}", error),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    /// - Ensure that the `object_name` matches the correct API name of the Salesforce object.
    /// - The `params` object should conform to the structure expected by the Salesforce API for the specified object.
    ///
    /// # Dependencies
    /// - Ensure the `T` type implements the `Serialize` trait (usually through a derived implementation).
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_basic_info_post.htm>
    pub async fn create_sobject<T: Serialize + Debug + SObject + SObjectOwner + Clone>(
        &mut self,
        mut record: T,
    ) -> Result<T, Error> {

        // Set the owner_id attribute to the authenticated user's ID if its None
        if record.get_owner_id().is_none() {
            record.set_owner_id(self.client.get_user_id().as_deref().map(str::to_string).as_deref());
        }

        let resource_url = format!(
            "{}/sobjects/{}",
            self.client.base_version_path()?,
            record.get_sobject_type()
        );
        let response = self.client.post(resource_url, record.clone(), vec![]).await?;
        let response: CreateResponse = handle_json_response(response).await?;
        if response.success {
            record.set_id(Some(&response.id));
        }
        // Fixme - should we throw an error if the response is not success?
        Ok(record)
    }

    /// Get Object Metadata Using sObject Basic Information
    ///
    /// Gets basic metadata for a specified object, including some object properties,
    /// recent items, and URIs for other resources related to the object.
    ///
    /// # Parameters
    /// - `object_name`: A string slice representing the name of the Salesforce object (SObject)
    ///   to describe. For example, "Account", "Contact", or a custom object like "CustomObject__c".
    ///
    /// # Returns
    /// - `Result<DescribeSObjectResult, Error>`:
    ///   - On success, returns a `DescribeSObjectResult` containing the metadata details
    ///     of the specified Salesforce object.
    ///   - On failure, returns an `Error` containing details about what went wrong.
    ///
    /// # Examples
    /// ```rust
    /// use rustsf::{Client, RestApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///     let describe_result = api.describe("Account").await?;
    ///     println!("Describe result: {:?}", describe_result);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Note
    /// To retrieve the complete metadata for an object, use the [describe_sobject](#method.describe_sobject) method
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_basic_info_get.htm>
    pub async fn describe(&mut self, object_name: &str) -> Result<SObjectInfo, Error> {
        let resource_url = format!(
            "{}/sobjects/{}",
            self.client.base_version_path()?,
            object_name
        );
        let response = self.client.get(resource_url, vec![], vec![]).await?;
        handle_json_response(response).await
    }

    /// Fetches the metadata description of a specified Salesforce object asynchronously.
    ///
    /// Completely describes the individual metadata at all levels for the given `object_name`.
    /// For example, this can be used to retrieve the fields, URLs, and child relationships
    /// for the Account object.
    /// The response is then processed and returned
    /// as a `DescribeResponse`.
    ///
    /// # Arguments
    ///
    /// * `object_name` - A string slice that specifies the API name of the Salesforce
    ///                   object to describe (e.g., "Account", "Contact").
    ///
    /// # Returns
    ///
    /// * `Ok(DescribeSObjectResult)` - If the operation is successful, returns a [DescribeSObjectResult](crate::rest_api::responses::describe_sobject_result)
    ///                             containing the object's metadata description.
    /// * `Err(Error)` - If an error occurs during the request or response processing,
    ///                  returns an `Error`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustsf::{Client, RestApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///
    ///     let object_name = "Account";
    ///     match api.describe_sobject(object_name).await {
    ///         Ok(response) => println!("Successfully retrieved object description: {:?}", response),
    ///         Err(e) => println!("Error retrieving object description: {:?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_describe.htm>
    pub async fn describe_sobject(
        &mut self,
        object_name: &str,
    ) -> Result<DescribeSObjectResult, Error> {
        let resource_url = format!(
            "{}/sobjects/{}/describe",
            self.client.base_version_path()?,
            object_name
        );
        let response = self.client.get(resource_url, vec![], vec![]).await?;
        handle_json_response(response).await
    }

    /// Get a List of Objects
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
    ///
    /// ```rust
    /// use rustsf::{Client, RestApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     use rustsf::RestApi;
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///
    ///     match api.describe_global().await {
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
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/dome_describeGlobal.htm>
    pub async fn describe_global(&mut self) -> Result<DescribeGlobalResponse, Error> {
        let resource_url = format!("{}/sobjects", self.client.base_version_path()?);
        let response = self.client.get(resource_url, vec![], vec![]).await?;
        handle_json_response(response).await
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
    /// use rustsf::{Client, RestApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     use rustsf::RestApi;
    ///     let client = Client::new();
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
    pub async fn describe_global_modified(
        &mut self,
        since: String,
    ) -> Result<DescribeGlobalResponse, Error> {
        let resource_url = format!("{}/sobjects", self.client.base_version_path()?);
        let response = self
            .client
            .get(
                resource_url,
                vec![],
                vec![("If-Modified-Since".to_string(), since)],
            )
            .await?;
        handle_json_response(response).await
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
    /// use rustsf::{Client, RestApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     use rustsf::RestApi;
    ///     let client = Client::new();
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
    pub async fn describe_global_unmodified(
        &mut self,
        since: String,
    ) -> Result<DescribeGlobalResponse, Error> {
        let resource_url = format!("{}/sobjects", self.client.base_version_path()?);
        let response = self
            .client
            .get(
                resource_url,
                vec![],
                vec![("If-Unmodified-Since".to_string(), since)],
            )
            .await?;
        handle_json_response(response).await
    }

    /// Deletes a specific record of a given Salesforce object type (`sobject_name`) using its ID.
    ///
    /// This asynchronous function constructs the resource URL for the given object type (`sobject_name`)
    /// and record ID (`id`), sends a `DELETE` request to the Salesforce API, and processes the response.
    ///
    /// # Arguments
    ///
    /// * `sobject_name` - The API name of the Salesforce object. For example, `"Account"`, `"Contact"`, etc.
    /// * `id` - The unique ID of the record to delete.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - Returns `Ok(())` if the record is successfully deleted.
    ///   Returns an `Err(Error)` if there is an issue during the deletion process (e.g., network error, API failure).
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// - If constructing the resource URL fails.
    /// - If the `DELETE` request to the Salesforce API fails.
    /// - If the response from the API indicates a failure or cannot be processed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustsf::{Client, RestApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///
    ///     let result = api.delete_sobject("Account", "001D000000IqhSLIAZ").await;
    ///
    ///     match result {
    ///         Ok(()) => println!("Record deleted successfully."),
    ///         Err(e) => eprintln!("Failed to delete record: {:?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Async
    ///
    /// This function is `async` and should be awaited.
    ///
    /// # Notes
    ///
    /// Ensure that the authenticated Salesforce client (`self.client`) has the necessary permissions
    /// to perform delete operations in the Salesforce org.
    pub async fn delete_sobject(&mut self, sobject_name: &str, id: &str) -> Result<(), Error> {
        let resource_url = format!(
            "{}/sobjects/{}/{}",
            self.client.base_version_path()?,
            sobject_name,
            id
        );
        let response = self.client.delete(resource_url).await?;
        handle_empty_response(response).await
    }

    /// Finds and retrieves a record of the specified Salesforce object type by its ID,
    /// containing all of its field values.
    ///
    /// This asynchronous function builds a resource URL using the provided Salesforce object name and
    /// record ID, sends a GET request to Salesforce, and deserializes the JSON response into the
    /// appropriate type.
    ///
    /// # Type Parameters
    /// * `T`: The type into which the returned JSON response will be deserialized.
    ///   Must implement the `DeserializeOwned` trait.
    ///
    /// # Parameters
    /// * `sobject_name`: A string slice representing the name of the Salesforce object (e.g., "Account", "Contact").
    /// * `id`: A string slice representing the unique ID of the Salesforce object record to retrieve.
    ///
    /// # Returns
    /// * On success: `Result<T, Error>` containing the deserialized response as type `T`.
    /// * On failure: `Result<T, Error>` containing an error if the request fails or if deserialization fails.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, RestApi, Error, DefSObject};
    ///
    /// #[DefSObject(sobject_type = "Account", fields="system,type,name")]
    /// struct Account { }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///
    ///     match api.fetch_by_id::<Account>("Account", "001D000000IqhSLIAZ").await {
    ///         Ok(record) => println!("Account Name: {:?}", record.name),
    ///         Err(error) => println!("Error retrieving account: {:?}", error),
    ///     };
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Dependencies
    /// This function depends on the client's `base_path()` method to obtain the base URL and the
    /// `get` method to perform the HTTP GET request. The response is then handled by the
    /// `handle_json_response` utility.
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_retrieve_get.htm>
    pub async fn fetch_by_id<T: DeserializeOwned>( // fixme rename into sobject_by_id
        &mut self,
        sobject_name: &str,
        id: &str,
    ) -> Result<T, Error> {
        let resource_url = format!(
            "{}/sobjects/{}/{}",
            self.client.base_version_path()?,
            sobject_name,
            id
        );
        let response = self.client.get(resource_url, vec![], vec![]).await?;

        // Adds the Attribute attribute to the response
        let mut attr = SObjectAttribute::new(sobject_name);
        attr.set_id(Some(id));
        let json = serde_json::to_value(attr).map_err(Error::from)?;

        let mut hm: HashMap<String, Value> = handle_json_response(response).await?;
        hm.insert("attributes".to_string(), json);
        let json = serde_json::to_value(hm).map_err(Error::from)?;
        serde_json::from_value(json).map_err(Error::from)
    }

    /// sObject Get Deleted
    ///
    /// Retrieves the list of individual records that have been deleted within the given timespan
    /// for the specified object (`sobject_name`).
    ///
    /// # Parameters
    /// - `sobject_name`: The API name of the Salesforce object whose deleted records are being queried.
    /// - `start_date`: Starting date/time (Coordinated Universal Time (UTC)—not local— timezone) of the
    ///   timespan for which to retrieve the data. The API ignores the seconds portion of the
    ///   specified dateTime value (for example, 12:30:15 is interpreted as 12:30:00 UTC).
    ///   The date and time must be formatted as described in Valid Date and DateTime Formats.
    ///   The date/time value for start must chronologically precede end.
    ///   This parameter should be URL-encoded.
    /// - `end_date`: Ending date/time (Coordinated Universal Time (UTC)—not local— timezone) of the
    ///   timespan for which to retrieve the data. The API ignores the seconds portion of the
    ///   specified dateTime value (for example, 12:35:15 is interpreted as 12:35:00 UTC).
    ///   The date and time must be formatted as described in Valid Date and DateTime Formats.
    ///   This parameter should be URL-encoded.
    ///
    /// # Returns
    /// - `Ok(DeletedSObjectsResponse)`: A structured response containing the details of the deleted objects.
    /// - `Err(Error)`: An error if the request fails or the API response is invalid.
    ///
    /// # Notes
    /// - Deleted records are written to a delete log which this resource accesses. A background
    ///   process that runs every two hours purges records that have been in an organization's
    ///   delete log for more than two hours if the number of records is above a certain limit.
    ///   Starting with the oldest records, the process purges delete log entries until the delete
    ///   log is back below the limit. This is done to protect Salesforce from performance issues
    ///   related to massive delete logs
    /// - Information on deleted records is returned only if the current session user has access to them.
    /// - Results are returned for no more than 15 days previous to the day the call is executed
    ///   (or earlier if an administrator has purged the Recycle Bin).
    /// - There is a limit of 600,000 IDs returned from this resource.
    ///   If more than 600,000 IDs are found, EXCEEDED_ID_LIMIT is returned.
    ///   You can correct the error by choosing start and end dates that are closer together.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, RestApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     use rustsf::RestApi;
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///     let start_date = "2023-01-01";
    ///     let end_date = "2023-01-31";
    ///     let result = api.sobject_get_deleted("Account", start_date, end_date).await;
    ///
    ///     match result {
    ///         Ok(response) => {
    ///             println!("Deleted objects: {:?}", response);
    ///         }
    ///         Err(e) => {
    ///             eprintln!("Error fetching deleted objects: {:?}", e);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_getdeleted.htm>
    pub async fn sobject_get_deleted(
        &mut self,
        sobject_name: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<DeletedSObjectsResponse, Error> {
        let resource_url = format!(
            "{}/sobjects/{}/deleted/",
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

    /// sObject Get Updated
    ///
    /// Retrieves the list of individual records that have been updated within the given timespan
    /// for the specified object (`sobject_name`).
    ///
    /// # Parameters
    /// - `sobject_name`: The API name of the Salesforce object whose updated records are being queried.
    /// - `start_date`: Starting date/time (Coordinated Universal Time (UTC)—not local— timezone) of the
    ///   timespan for which to retrieve the data. The API ignores the seconds portion of the
    ///   specified dateTime value (for example, 12:30:15 is interpreted as 12:30:00 UTC).
    ///   The date and time must be formatted as described in Valid Date and DateTime Formats.
    ///   The date/time value for start must chronologically precede end.
    ///   This parameter should be URL-encoded.
    /// - `end_date`: Ending date/time (Coordinated Universal Time (UTC)—not local— timezone) of the
    ///   timespan for which to retrieve the data. The API ignores the seconds portion of the
    ///   specified dateTime value (for example, 12:35:15 is interpreted as 12:35:00 UTC).
    ///   The date and time must be formatted as described in Valid Date and DateTime Formats.
    ///   This parameter should be URL-encoded.
    ///
    /// # Returns
    /// - `Ok(UpdatedSObjectsResponse)`: A structured response containing the details of the updated objects.
    /// - `Err(Error)`: An error if the request fails or the API response is invalid.
    ///
    /// # Notes
    /// - Deleted records are written to a delete log which this resource accesses. A background
    ///   process that runs every two hours purges records that have been in an organization's
    ///   delete log for more than two hours if the number of records is above a certain limit.
    ///   Starting with the oldest records, the process purges delete log entries until the delete
    ///   log is back below the limit. This is done to protect Salesforce from performance issues
    ///   related to massive delete logs
    /// - Information on deleted records is returned only if the current session user has access to them.
    /// - Results are returned for no more than 15 days previous to the day the call is executed
    ///   (or earlier if an administrator has purged the Recycle Bin).
    /// - There is a limit of 600,000 IDs returned from this resource.
    ///   If more than 600,000 IDs are found, EXCEEDED_ID_LIMIT is returned.
    ///   You can correct the error by choosing start and end dates that are closer together.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, RestApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     use rustsf::RestApi;
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///     let start_date = "2023-01-01";
    ///     let end_date = "2023-01-31";
    ///     let result = api.sobject_get_deleted("Account", start_date, end_date).await;
    ///
    ///     match result {
    ///         Ok(response) => {
    ///             println!("Deleted objects: {:?}", response);
    ///         }
    ///         Err(e) => {
    ///             eprintln!("Error fetching deleted objects: {:?}", e);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_getupdated.htm>
    pub async fn sobject_get_updated(
        &mut self,
        sobject_name: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<UpdatedSObjectsResponse, Error> {
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
    /// use rustsf::{Client, RestApi, Error, DefSObject};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[DefSObject(sobject_type = "Account", fields="system,type,name")]
    /// struct Account { }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
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
    ) -> Result<(), Error> {
        let resource_url = format!(
            "{}/sobjects/{}/{}",
            self.client.base_version_path()?,
            object_name,
            id
        );
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
    /// use rustsf::{Client, RestApi, Error};
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
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
    ) -> Result<Response, Error> {
        let resource_url = format!(
            "{}/sobjects/{}/{}/{}",
            self.client.base_version_path()?,
            sobject_name,
            key_name,
            key
        );
        self.client.patch(resource_url, params).await
    }
}