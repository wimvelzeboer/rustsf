//! Module containing all the SOQL query methods,
//! by calling the Salesforce endpoints `/services/data/vXX.X/query` and `/services/data/vXX.X/queryAll`.
//!
//! # See
//! <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_query.htm>

use super::{RestApi, handle_json_response};
use crate::Error;
use crate::rest_api::responses::query_response::QueryResponse;
use serde::de::DeserializeOwned;

impl RestApi {
    /// Executes an asynchronous query against the client's configured base path and returns the parsed response.
    /// Up to 2,000 records can be returned at a time in a request
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
    /// use rustsf::{Client, RestApi, Error, DefSObject};
    ///
    /// #[DefSObject(sobject_type = "Account", fields="system,audit,type,name")]
    /// struct Account { }
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
    /// - tag query
    pub async fn query<T: DeserializeOwned>(
        &mut self,
        query: &str,
    ) -> Result<QueryResponse<T>, Error> {
        let query_url = format!("{}/query/", self.client.base_version_path()?);
        let params = vec![("q".to_string(), query.to_string())];
        let response = self.client.get(query_url, params, vec![]).await?;
        handle_json_response(response).await
    }

    /// Executes a query to retrieve all matching results from the server.
    /// Unlike the query method, query_all returns records that are soft deleted due to a merge
    /// or delete. After these records are permanently removed from the recycle bin, you can no
    /// longer query them. QueryAll also returns information about archived task and event records.
    /// Up to 2,000 records can be returned at a time in a request
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
    /// use rustsf::{Client, RestApi, Error, DefSObject};
    ///
    /// #[DefSObject(sobject_type = "Account", fields="system,type,audit,name")]
    /// struct Account { }
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
    pub async fn query_all<T: DeserializeOwned>(
        &mut self,
        query: &str,
    ) -> Result<QueryResponse<T>, Error> {
        let query_url = format!("{}/queryAll/", self.client.base_version_path()?);
        let params = vec![("q".to_string(), query.to_string())];
        let response = self.client.get(query_url, params, vec![]).await?;
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
    /// use rustsf::{Client, RestApi, Error, DefSObject};
    ///
    /// #[DefSObject(sobject_type = "Account", fields="system,type,audit,name")]
    /// struct Account { }
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
    pub async fn query_more<T: DeserializeOwned>(
        &mut self,
        next_records_url: &str,
    ) -> Result<QueryResponse<T>, Error> {
        let instance_url = self
            .client
            .instance_url
            .as_ref()
            .ok_or(Error::NotLoggedIn)?;
        let query_url = format!("{}/{}", instance_url, next_records_url);
        let response = self.client.get(query_url, vec![], vec![]).await?;
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
    /// use rustsf::{Client, RestApi, Error, DefSObject};
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
    ///     let next_url = "/services/data/v67.0/queryAll/01gB0000003mzKJQAY-2000";
    ///     match api.query_all_more::<Account>(&next_url).await {
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
    pub async fn query_all_more<T: DeserializeOwned>(
        &mut self,
        next_records_url: &str,
    ) -> Result<QueryResponse<T>, Error> {
        let instance_url = self
            .client
            .instance_url
            .as_ref()
            .ok_or(Error::NotLoggedIn)?;
        let query_url = format!("{}/{}", instance_url, next_records_url);
        let response = self.client.get(query_url, vec![], vec![]).await?;
        handle_json_response(response).await
    }
}
