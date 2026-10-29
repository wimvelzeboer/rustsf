//! # Search SOSL API
//!
//! Module containing all the Search SOSL query methods
//!
//!  ## Supported Endpoints
//! - **/services/data/vXX.X/search/**
//!
//! ## Methods
//! - [**search_sosl**](crate::rest_api::RestApi#method.search_sosl), executes a SOSL query
//!
//! # See
//! <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_search.htm>
//!

use crate::rest_api::builder::RestApiBuilder;
use crate::rest_api::builder::search::sosl_response::SearchResponse;
use crate::errors::Error;
use crate::rest_api::HTTPMethod;
use crate::RestApi;
use self::sosl_response::SoslResponse;

mod sosl_response;
mod search_record;

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
	/// use rustsf::{RestApi, SalesforceCredentials};
	/// use chrono::Utc;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let credentials = SalesforceCredentials::sfdx_url("force://PlatformCLI::token@example.com").unwrap();
	///     let api = RestApi::new(credentials).await.unwrap();
	///
	///     let query = "FIND {test} RETURNING Account(Name)";
	///
	///     match api.search_sosl(query).await {
	///         Ok(response) => println!("Search results: {:?}", response),
	///         Err(error) => println!("Error executing SOSL query: {:?}", error),
	///     }
	/// }
	/// ```
	///
	/// # Notes
	/// - Before executing this function, ensure that the `SalesforceClient` is properly authenticated.
	/// - This function utilizes the Salesforce REST API `/search/` endpoint.
	pub async fn search_sosl(&self, query: &str) -> Result<SearchResponse, Error> {
		Ok(self.build()
			.sosl(query)
			.send()
			.await?
			.take_response())
	}
}

impl<'a> RestApiBuilder<'a> {
	pub fn sosl<'b>(&'a self, query: &'b str) -> SoslBuilder<'a, 'b> {
		SoslBuilder::new(&self, query)
	}
}

pub struct SoslBuilder<'a, 'b> {
	rest_api_builder: &'a RestApiBuilder<'a>,
	query: &'b str,
}

impl<'a, 'b> SoslBuilder<'a, 'b> {
	pub fn new(rest_api_builder: &'a RestApiBuilder, query: &'b str) -> SoslBuilder<'a, 'b> {
		SoslBuilder {
			rest_api_builder,
			query,
		}
	}

	pub async fn send(&self) -> Result<SoslResponse, Error> {
		let url = "search".to_string();
		let params = vec![("q".to_string(), self.query.to_string())];
		let response = self.rest_api_builder.send(
			HTTPMethod::GET, url, &params).await?;
		Ok(SoslResponse::new(RestApi::handle_json_response(response).await?))
	}
}
