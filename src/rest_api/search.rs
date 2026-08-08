//! Module containing all the Search SOSL query methods,
//! by calling the Salesforce endpoint `/services/data/vXX.X/search/`.
//!
//! # See
//! <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_search.htm>
//!

use super::{RestApi, handle_json_response};
use crate::Error;
use crate::rest_api::responses::search_response::SearchResponse;

impl RestApi {
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
        let query_url = format!("{}/search/", self.client.base_version_path()?);
        let params = vec![("q".to_string(), query.to_string())];
        let response = self.client.get(query_url, params, vec![]).await?;
        handle_json_response(response).await
    }
}
