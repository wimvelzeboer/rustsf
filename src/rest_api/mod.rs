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

use crate::authentication::{Client, Credentials};
use std::collections::HashMap;
use std::fmt::Debug;
// use crate::rest_api::query::RestApiQueryBuilder;
use crate::rest_api::builder::RestApiBuilder;
use anyhow::{Context, Result, anyhow};
use reqwest::Response;
use responses::error_response::ErrorResponse;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub mod builder;
pub mod responses;

pub enum HTTPMethod {
	GET,
	POST,
	// PUT,
	// DELETE,
	// PATCH,
	// OPTIONS,
	// HEAD,
	// CONNECT,
}

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
	/// use rustsf::{AuthUrl, RestApi};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///
	///     let mut api = RestApi::new(AuthUrl::new("force://......."));
	///     // you call the api here..
	///
	///     Ok(())
	/// }
	/// ```
	pub async fn new(credentials: Credentials) -> Result<Self> {
		Ok(Self {
			client: Client::connect(credentials)
				.await
				.context("Failed to connect to Salesforce REST API")?,
		})
	}

	pub(crate) async fn send_delete(
		&self,
		url: String,
		headers: &HashMap<String, String>,
	) -> Result<reqwest::Response> {
		let url = format!("{}/{}", self.client.instance_url(), url);
		self.client.delete(&url, &headers).await
	}

	pub(crate) async fn send<T: Debug + Serialize>(
		&self,
		method: HTTPMethod,
		url: String,
		data: &T,
		headers: &HashMap<String, String>,
	) -> Result<reqwest::Response> {
		let url = format!("{}/{}", self.client.instance_url(), url);
		match method {
			HTTPMethod::GET => Ok(self.client.get(&url, data, &headers).await?),
			HTTPMethod::POST => Ok(self.client.post(&url, data, &headers).await?),
		}
	}

	// todo add method for unsupported paths
}

// pub mod composite;
// pub mod query;
// pub mod search;
// pub mod services;

#[cfg(test)]
mod test;
// pub mod user_password;

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
