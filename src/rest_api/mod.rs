//! Salesforce REST API
//!
//! This crate provides a Rust client for interacting with the Salesforce REST API. It allows you to
//! perform CRUD operations on Salesforce objects, query data, and more.
//!
//! ## Supported Endpoint
//!
//! - [**/services/data/vXX.X/sobjects**](crate::rest_api::sobjects), CRUD operations on a single Salesforce object
//! - [**/services/data/vXX.X/composite/sobjects**](crate::rest_api::composite), CRUD operations for up to 200 Salesforce objects
//! - [**/services/data/vXX.X/search**](crate::rest_api::search), SOSL search operations on Salesforce objects
//! - [**/services/data/vXX.X/services**](crate::rest_api::services), system information and metadata
//! - [**/services/data/vXX.X/query**](crate::rest_api::query), retrieving up to 2,000 records
//! - [**/services/data/vXX.X/userPassword**](crate::rest_api::user_password), user password authentication

use crate::client::client::Client;
use anyhow::{Result, anyhow};
use reqwest::Response;
use responses::error_response::ErrorResponse;
use serde::de::DeserializeOwned;

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
pub struct RestApi {
	pub(crate) client: Client,
}

async fn handle_json_response<T: DeserializeOwned>(response: Response) -> Result<T> {
	if response.status().is_success() {
		Ok(response.json().await?)
	} else {
		let errors: Vec<ErrorResponse> = response.json().await?;
		Err(anyhow!("Response error {:?}", errors))
	}
}

async fn handle_empty_response(response: Response) -> Result<()> {
	if response.status().is_success() {
		Ok(())
	} else {
		let errors: Vec<ErrorResponse> = response.json().await?;
		Err(anyhow!("Response error {:?}", errors))
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
	/// use rustsf::{Client, Credentials, RestApi};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let client= Client::new(Credentials::new()).await?;
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
}

pub mod composite;
pub mod query;
pub mod search;
pub mod services;
pub mod sobjects;
#[cfg(test)]
mod test;
pub mod user_password;
