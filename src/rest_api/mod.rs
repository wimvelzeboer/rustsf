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
}

#[cfg(test)]
mod test;
pub mod sobjects;
pub mod composite;
pub mod query;
pub mod search;
pub mod services;
pub mod user_password;