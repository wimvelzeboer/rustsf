//! Salesforce REST API
//!
//! This crate provides a Rust client for interacting with the Salesforce REST API. It allows you to
//! perform CRUD operations on Salesforce objects, query data, and more.
//!
//! ## Features
//! - **Asynchronous** - All functions are asynchronous and return a `Future` that can be awaited.
//! - **Error Handling** - All functions return `Result`s, allowing you to handle errors gracefully.
//! - **Type-Safe** - All API endpoints are typed, ensuring that you are sending the correct data and
//!   receiving the expected response.
//!

use crate::client::client::Client;
use crate::errors::Error;
use responses::error_response::ErrorResponse;
use reqwest::Response;
use serde::Serialize;
use serde::de::DeserializeOwned;
use responses::create_response::CreateResponse;
use responses::describe_global_response::DescribeGlobalResponse;
use responses::describe_response::DescribeResponse;
use responses::query_response::QueryResponse;
use responses::search_response::SearchResponse;
use responses::version_response::VersionResponse;

pub mod responses;

/// A `RestApi` struct that represents the core component for interacting with a RESTful API.
///
/// This struct contains a `client` field, which is an instance of the `Client` type
/// used to make HTTP requests to the API. The `RestApi` struct is derived with the
/// `Default` trait, allowing it to be instantiated with default values.
///
/// # Fields
/// - `client` (`Client`):
///   A structure responsible for handling HTTP interactions with the API.
///   This field has module-level visibility (`pub(crate)`), restricting its accessibility
///   to the current crate.
///
/// Note: To use the `RestApi` struct effectively, ensure all required components
/// of the `Client` type are configured appropriately.
#[derive(Default)]
pub struct RestApi {
    pub(crate) client: Client,
}

async fn handle_json_response<T: DeserializeOwned>(response: Response) -> Result<T, Error> {    if response.status().is_success() {
        Ok(response.json().await?)
    } else {
        let errors: Vec<ErrorResponse> = response.json().await?;
        Err(Error::ErrorResponses(errors))
    }
}

async fn handle_empty_response(response: Response) -> Result<(), Error> {
    if response.status().is_success() {
        Ok(())
    } else {
        let errors: Vec<ErrorResponse> = response.json().await?;
        Err(Error::ErrorResponses(errors))
    }
}

impl RestApi {
    /// Creates a new instance of the `RestApi` struct with the provided `Client`.
    ///
    /// # Arguments
    ///
    /// * `client` - An instance of the `Client` that this `RestApi` will use to perform API requests.
    ///
    /// # Returns
    ///
    /// A new `RestApi` instance initialized with the given `Client`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustsf::{Client, RestApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///     // you call the api here..
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(client: Client) -> Self {
        RestApi { client }
    }

    /// Executes an asynchronous query against the client's configured base path and returns the parsed response.
    ///
    /// # Type Parameters
    /// - `T`: The type to which the JSON response will be deserialized. This type must implement the `DeserializeOwned` trait from Serde.
    ///
    /// # Arguments
    /// - `query`: A string slice that holds the query to be executed.
    ///
    /// # Returns
    /// - `Ok(QueryResponse<T>)` if the query executes successfully and the response is successfully deserialized.
    /// - `Err(Error)` if an error occurs during the process, such as an HTTP request failure or JSON deserialization error.
    ///
    /// # Errors
    /// This function returns an `Error` in the following situations:
    /// - If the base path of the client cannot be determined.
    /// - If the HTTP GET request fails.
    /// - If the HTTP response cannot be parsed into the desired type `T`.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, RestApi, Error};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Debug, Deserialize, Serialize)]
    /// #[serde(rename_all = "PascalCase")]
    /// struct Account {
    ///     id: String,
    ///     name: String,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///     let query_str = "SELECT Id, Name FROM Account WHERE Id = '001D000000IqhSLIAZ'";
    ///     match api.query::<Account>(query_str).await {
    ///         Ok(response) => println!("Query response: {:?}", response),
    ///         Err(error) => println!("Error executing query: {:?}", error),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn query<T: DeserializeOwned>(&mut self, query: &str) -> Result<QueryResponse<T>, Error> {
        let query_url = format!("{}/query/", self.client.base_path()?);
        let params = vec![("q".to_string(), query.to_string())];
        let response = self.client.get(query_url, params).await?;
        handle_json_response(response).await
    }

    /// Executes a query to retrieve all matching results from the server.
    ///
    /// # Type Parameters
    /// - `T`: The type to which the query response will be deserialized. `T` must implement `DeserializeOwned`.
    ///
    /// # Arguments
    /// - `query`: A string slice containing the query to execute.
    ///
    /// # Returns
    /// Returns a `Result` containing either:
    /// - `QueryResponse<T>`: A structured response where the data is deserialized into the specified type `T`.
    /// - `Error`: An error if the query execution or deserialization fails.
    ///
    /// # Errors
    /// - Returns an error if the client base path cannot be retrieved.
    /// - Returns an error if the HTTP `GET` request fails.
    /// - Returns an error if the JSON response cannot be handled or deserialized into the specified type `T`.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, RestApi, Error};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Debug, Deserialize, Serialize)]
    /// #[serde(rename_all = "PascalCase")]
    /// struct Account {
    ///     id: String,
    ///     name: String,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///     let query = "SELECT * FROM Account";
    ///     match api.query_all::<Account>(query).await {
    ///         Ok(response) => println!("Query response: {:?}", response),
    ///         Err(error) => println!("Error executing query: {:?}", error),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn query_all<T: DeserializeOwned>(&mut self, query: &str) -> Result<QueryResponse<T>, Error> {
        let query_url = format!("{}/queryAll/", self.client.base_path()?);
        let params = vec![("q".to_string(), query.to_string())];
        let response = self.client.get(query_url, params).await?;
        handle_json_response(response).await
    }

    /// Executes a query to retrieve additional records from a Salesforce API endpoint.
    ///
    /// This function is used to fetch subsequent records from a paginated Salesforce query response.
    /// The `next_records_url` provided is the URL for the next set of results as returned by the
    /// Salesforce API in a previous query.
    ///
    /// # Type Parameters
    /// - `T`: The type to which the query response will be deserialized.
    ///   It must implement the `DeserializeOwned` trait.
    ///
    /// # Arguments
    /// - `next_records_url`: A reference to a string slice representing the URL for the next set
    ///   of records in the paginated response.
    ///
    /// # Returns
    /// - `Ok(QueryResponse<T>)`: On success, returns a `QueryResponse<T>` containing the deserialized
    ///   response data.
    /// - `Err(Error)`: If an error occurs, returns an `Error` detailing the cause. This may include:
    ///   - `Error::NotLoggedIn`: If the client is not authenticated or lacks an instance URL.
    ///   - Any other error encountered during the HTTP request or JSON deserialization.
    ///
    /// # Errors
    /// This function will return an error if:
    /// - The client is not logged in, and thus no instance URL is available.
    /// - The HTTP request to the provided `next_records_url` fails.
    /// - The JSON response cannot be deserialized into the specified type `T`.
    ///
    /// # Examples
    /// ```rust
    /// use rustsf::{Client, RestApi, Error};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Debug, Deserialize, Serialize)]
    /// #[serde(rename_all = "PascalCase")]
    /// struct Account {
    ///     id: String,
    ///     name: String,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///     let next_url = "/services/data/v60.0/query/01gB0000003mzKJQAY-2000";
    ///     match api.query_more::<Account>(&next_url).await {
    ///         Ok(response) => println!("Query response: {:?}", response),
    ///         Err(error) => println!("Error executing query: {:?}", error),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    /// - The function assumes the `next_records_url` is a relative path (e.g., `/services/data/...`)
    ///   that will be appended to the Salesforce instance URL.
    /// - This function performs an asynchronous HTTP GET request and requires an asynchronous runtime to execute.
    ///
    /// # Dependencies
    /// - The `handle_json_response` function is used to parse and handle the JSON response from the API.
    /// - Assumes `self.client.get` performs an HTTP GET request and returns the raw HTTP response.
    pub async fn query_more<T: DeserializeOwned>(&mut self, next_records_url: &str) -> Result<QueryResponse<T>, Error> {
        let instance_url = self
            .client
            .instance_url
            .as_ref()
            .ok_or(Error::NotLoggedIn)?;
        let query_url = format!("{}/{}", instance_url, next_records_url);
        let response = self.client.get(query_url, vec![]).await?;
        handle_json_response(response).await
    }

    /// Executes a SOSL (Salesforce Object Search Language) query and returns the search results.
    ///
    /// # Parameters
    /// - `query`: A string slice containing the SOSL query to be executed.
    ///
    /// # Returns
    /// - `Result<SearchResponse, Error>`:
    ///   - On success: A `SearchResponse` object containing the results of the SOSL query.
    ///   - On failure: An `Error` indicating the cause of the failure.
    ///
    /// # Errors
    /// This function may return an error if:
    /// - There is an issue with constructing the query URL.
    /// - The HTTP GET request to the Salesforce API fails.
    /// - The response from the server cannot be properly parsed or contains an error.
    ///
    /// # Examples
    /// ```rust
    /// use rustsf::{Client, RestApi, Error};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///     let query = "FIND {test} RETURNING Account(Name)";
    ///     let result = api.search_sosl(query).await;
    ///     match result {
    ///         Ok(response) => println!("Search results: {:?}", response),
    ///         Err(error) => println!("Error executing SOSL query: {:?}", error),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    /// - Before executing this function, ensure that the `SalesforceClient` is properly authenticated.
    /// - This function utilizes the Salesforce REST API `/search/` endpoint.
    pub async fn search_sosl(&mut self, query: &str) -> Result<SearchResponse, Error> {
        let query_url = format!("{}/search/", self.client.base_path()?);
        let params = vec![("q".to_string(), query.to_string())];
        let response = self.client.get(query_url, params).await?;
        handle_json_response(response).await
    }

    /// Retrieves a list of Salesforce API versions available for the authenticated instance.
    ///
    /// # Returns
    ///
    /// An asynchronous function that returns a `Result`:
    /// - `Ok(Vec<VersionResponse>)` on success, containing a vector of `VersionResponse` objects
    ///   that encapsulate the details of each available API version.
    /// - `Err(Error)` on failure, which can occur for the following reasons:
    ///   - `Error::NotLoggedIn`: If the client is not authenticated with a valid instance URL.
    ///   - Other errors that may arise from HTTP request failures or JSON deserialization.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The client instance URL (`self.client.instance_url`) is not set, which indicates that the user is not logged in.
    /// - There is an issue performing the HTTP GET request to fetch the versions.
    /// - The server response fails to deserialize into the expected format.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, RestApi, Error};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///     let result = api.versions().await;
    ///     match result {
    ///         Ok(versions) => {
    ///             for version in versions {
    ///                 println!("Version: {}", version.label);
    ///             }
    ///         }
    ///         Err(error) => eprintln!("Error retrieving versions: {:?}", error),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Implementation Details
    ///
    /// 1. Verifies that the client is logged in by checking whether the `instance_url` is set.
    /// 2. Constructs the URL for retrieving API versions (`/services/data/` under the instance URL).
    /// 3. Sends an HTTP GET request to the constructed URL.
    /// 4. Processes the response using the `handle_json_response` function to deserialize and return the parsed output.
    pub async fn versions(&mut self) -> Result<Vec<VersionResponse>, Error> {
        let instance_url = match self.client.instance_url.as_ref() {
            Some(url) => url,
            None => return Err(Error::NotLoggedIn),
        };
        let versions_url = format!("{}/services/data/", instance_url);
        let response = self.client.get(versions_url, vec![]).await?;
        handle_json_response(response).await
    }

    /// Finds and retrieves a record of the specified Salesforce object type by its ID.
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
    /// # Errors
    /// * Returns an error if there are issues with building the resource URL.
    /// * Returns an error if the HTTP GET request fails.
    /// * Returns an error if deserialization of the JSON response into type `T` fails.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, RestApi, Error};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Deserialize, Serialize)]
    /// #[serde(rename_all = "PascalCase")]
    /// struct Account {
    ///     id: String,
    ///     name: String,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///
    ///     match api.find_by_id::<Account>("Account", "001D000000IqhSLIAZ").await {
    ///         Ok(record) => println!("Account Name: {}", record.name),
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
    pub async fn find_by_id<T :DeserializeOwned>(
        &mut self,
        sobject_name: &str,
        id: &str,
    ) -> Result<T, Error> {
        let resource_url = format!("{}/sobjects/{}/{}", self.client.base_path()?, sobject_name, id);
        let response = self.client.get(resource_url, vec![]).await?;
        handle_json_response(response).await
    }

    /// Asynchronously creates a new record in a Salesforce object using the provided parameters.
    ///
    /// # Generic
    /// - `T`: A type that implements the `Serialize` trait, representing the parameters for the new record.
    ///
    /// # Arguments
    /// - `object_name`: A string slice that holds the API name of the Salesforce object (e.g., "Account", "Contact").
    /// - `params`: An instance of type `T` containing the details of the record to be created.
    ///
    /// # Returns
    /// - `Result<CreateResponse, Error>`:
    ///     - On success, returns a `CreateResponse` containing information about the created record, such as its ID.
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
    /// use rustsf::{Client, RestApi, Error};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Deserialize, Serialize)]
    /// #[serde(rename_all = "PascalCase")]
    /// struct Account {
    ///     name: String,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///     let account = Account {
    ///         name: "Example Account".to_string(),
    ///     };
    ///     match api.create("Account", account).await {
    ///         Ok(response) => println!("Record ID: {}", response.id),
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
    pub async fn create<T: Serialize>(
        &mut self,
        object_name: &str,
        params: T,
    ) -> Result<CreateResponse, Error> {
        let resource_url = format!("{}/sobjects/{}", self.client.base_path()?, object_name);
        let response = self.client.post(resource_url, params, vec![]).await?;
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
    /// use rustsf::{Client, RestApi, Error};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Deserialize, Serialize)]
    /// #[serde(rename_all = "PascalCase")]
    /// struct Account {
    ///     id: String,
    ///     name: String,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = RestApi::new(client);
    ///     let account = Account {
    ///         id: "001D000000IqhSLIAZ".to_string(),
    ///         name: "Updated Account Name".to_string(),
    ///     };
    ///     match api.update("Account", "001D000000IqhSLIAZ", account).await {
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
    pub async fn update<T: Serialize>(
        &mut self,
        object_name: &str,
        id: &str,
        params: T,
    ) -> Result<(), Error> {
        let resource_url = format!("{}/sobjects/{}/{}", self.client.base_path()?, object_name, id);
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
    /// use serde::{Deserialize, Serialize};
    /// use serde_json::json;
    ///
    /// #[derive(Deserialize, Serialize)]
    /// #[serde(rename_all = "PascalCase")]
    /// struct Account {
    ///     id: String,
    ///     name: String,
    /// }
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
    ///     let result = api.upsert(
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
    pub async fn upsert<T: Serialize>(
        &mut self,
        sobject_name: &str,
        key_name: &str,
        key: &str,
        params: T,
    ) -> Result<Response, Error> {
        let resource_url = format!(
            "{}/sobjects/{}/{}/{}",
            self.client.base_path()?,
            sobject_name,
            key_name,
            key
        );
        self.client.patch(resource_url, params).await
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
    ///     let result = api.destroy("Account", "001D000000IqhSLIAZ").await;
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
    pub async fn destroy(&mut self, sobject_name: &str, id: &str) -> Result<(), Error> {
        let resource_url = format!("{}/sobjects/{}/{}", self.client.base_path()?, sobject_name, id);
        let response = self.client.delete(resource_url).await?;
        handle_empty_response(response).await
    }

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
    pub async fn describe_global(&mut self) -> Result<DescribeGlobalResponse, Error> {
        let resource_url = format!("{}/sobjects", self.client.base_path()?);
        let response = self.client.get(resource_url, vec![]).await?;
        handle_json_response(response).await
    }

    /// Fetches the metadata description of a specified Salesforce object asynchronously.
    ///
    /// This function constructs a resource URL for the given `object_name` and sends
    /// a GET request to retrieve the object's metadata description, such as its fields,
    /// relationships, and other details. The response is then processed and returned
    /// as a `DescribeResponse`.
    ///
    /// # Arguments
    ///
    /// * `object_name` - A string slice that specifies the API name of the Salesforce
    ///                   object to describe (e.g., "Account", "Contact").
    ///
    /// # Returns
    ///
    /// * `Ok(DescribeResponse)` - If the operation is successful, returns a `DescribeResponse`
    ///                             containing the object's metadata description.
    /// * `Err(Error)` - If an error occurs during the request or response processing,
    ///                  returns an `Error`.
    ///
    /// # Errors
    ///
    /// This function may return errors in the following situations:
    /// - If constructing the resource URL fails.
    /// - If the GET request fails or returns an unexpected response.
    /// - If decoding or handling the JSON response fails.
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
    ///     match api.describe(object_name).await {
    ///         Ok(response) => println!("Successfully retrieved object description: {:?}", response),
    ///         Err(e) => println!("Error retrieving object description: {:?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn describe(&mut self, object_name: &str) -> Result<DescribeResponse, Error> {
        let resource_url = format!("{}/sobjects/{}/describe", self.client.base_path()?, object_name);
        let response = self.client.get(resource_url, vec![]).await?;
        handle_json_response(response).await
    }
}

#[cfg(test)]
mod test;