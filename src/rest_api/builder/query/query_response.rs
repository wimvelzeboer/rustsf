use std::collections::HashMap;
use crate::rest_api::HTTPMethod;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;
use crate::errors::Error;
use crate::RestApi;

pub struct QueryResponse<'a> {
	rest_api: &'a RestApi,
	pub response: SalesforceQueryResponse<Value>,
}

impl<'a> QueryResponse<'a> {
	pub fn new(rest_api: &'a RestApi, response: SalesforceQueryResponse<Value>) -> Self {
		Self {
			rest_api,
			response,
		}
	}

	pub fn done(&self) -> bool {
		self.response.done
	}

	pub fn records<T: DeserializeOwned>(self) -> Result<Vec<T>, Error> {
		let records = self.response.records;
		let sobjects: Vec<T> = serde_json::from_value(records)?;
		Ok(sobjects)
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
	/// use rustsf::{Client, Credentials, RestApi, DefSObject};
	/// use anyhow::Result;
	///
	/// #[DefSObject(sobject_type = "Account", fields="system,type,audit,name")]
	/// struct Account { }
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
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
	pub async fn send_query_more(&mut self) -> Result<&mut Self, Error> {
		match &self.response.next_records_url {
			Some(url) => {
				let params: Vec<(String, String)> = vec![];
				let response = self.rest_api.send(HTTPMethod::GET, url.to_string(), &params, &HashMap::new()).await?;
				self.response = RestApi::handle_json_response(response).await?;
			}
			None => {}
		}
		Ok(self)
	}

	pub fn total_size(&self) -> i32 {
		self.response.total_size
	}

	pub fn next_records_url(&self) -> Option<&str> {
		self.response.next_records_url.as_deref()
	}

	pub fn take_response<T: DeserializeOwned>(self) -> Result<SalesforceQueryResponse<Vec<T>>, Error> {
		Ok(SalesforceQueryResponse {
			total_size: self.response.total_size,
			done: self.response.done,
			next_records_url: self.response.next_records_url,
			records: serde_json::from_value(self.response.records)?
		})
	}
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SalesforceQueryResponse<T> {
	pub total_size: i32,
	pub done: bool,
	pub next_records_url: Option<String>,
	pub records: T,
}