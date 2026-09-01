//! Salesforce API Client
//!
//! This crate provides a client with several authentication types for the Salesforce APIs.
//!

use crate::client::responses::access_token::AccessToken;
use crate::client::responses::token_response::TokenResponse;
use anyhow::{Context, Result, anyhow};
use log::debug;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderName, HeaderValue};
use reqwest::{Response, Url};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::credentials::{Credentials, CredentialsType};
#[cfg(feature = "metadata-api")]
use reqwest::multipart::Form;

/// Represents a client used for interacting with a remote API.
/// This struct encapsulates all the necessary data required to authenticate
/// and send HTTP requests to the API.
///
/// # Fields
///
/// * `http_client` - The underlying HTTP client (`reqwest::Client`) used to execute requests.
/// * `client_id` - An optional client ID used for authentication.
/// * `client_secret` - An optional client secret used for authentication.
/// * `login_endpoint` - The URL of the login endpoint used for authenticating requests.
/// * `instance_url` - An optional URL of the API instance to which the client is connected.
/// * `access_token` - An optional access token representing the user session for authenticated requests.
/// * `refresh_token` - An optional refresh token used to obtain a new access token when it expires.
/// * `version` - The version of the API being used.
/// * `secret_required` - A boolean value indicating whether the client secret is mandatory for authentication.
///
/// # Derive Attributes
///
/// * `Clone` - Enables creating a deep copy of the `Client`.
/// * `Debug` - Enables formatting a debugging representation of the `Client`.
///
/// # Usage
///
/// This struct is designed to facilitate authenticated communication with an API.
/// Use it to configure connection settings, store authentication tokens,
/// and manage HTTP request creation.
///
/// Example usage may include setting up the client with credentials, defining the
/// API endpoint, and invoking authenticated API calls with the underlying HTTP client.
///
/// # Example
/// ```rust
/// use rustsf::{Client, Credentials};
/// use anyhow::Result;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///
///     // Configure authentication credentials, e.g.:
///     let mut credentials = Credentials::new();
///     credentials.set_client_id("client_Id");
///     credentials.set_client_secret("client_secret");
///
///     let client = Client::new(Credentials::new()).await;
///
///     match client {
///         Ok(_) => println!("Login successful!"),
///         Err(e) => println!("Login failed: {}", e),
///     }
///     // ... other logic ...
///     Ok(())
/// }
/// ```
///
/// # Note
///
/// Ensure that sensitive fields like `client_secret` and `access_token` are handled securely
/// to avoid unintended exposure of confidential data.
#[derive(Debug)]
pub struct Client {
	pub http_client: reqwest::Client,
	pub(crate) credentials: Credentials,
	pub(crate) version: String,
}

impl Client {
	/// Creates a new `Client` instance with default configuration.
	///
	/// This function initializes a `Client` with the following default properties:
	/// - A new `reqwest::Client` instance for making HTTP requests.
	/// - `client_id`: `None`
	/// - `client_secret`: `None`
	/// - `login_endpoint`: Set to `"https://login.salesforce.com"`.
	/// - `access_token`: `None`
	/// - `instance_url`: `None`
	/// - `refresh_token`: `None`
	/// - `secret_required`: Set to `true`.
	/// - `version`: Set to `"v60.0"`.
	///
	/// # Returns
	/// A configured `Client` instance ready for use.
	///
	/// # Examples
	/// ```
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client = Client::new(Credentials::new()).await?;
	///     Ok(())
	/// }
	/// ```
	pub async fn new(credentials: Credentials) -> Result<Client> {
		let http_client = reqwest::Client::new();

		let credentials = match credentials.get_flow_type() {
			Some(flow_type) => match flow_type {
				CredentialsType::AuthUrl => login_with_sfdx_auth_url(credentials)
					.await
					.context("Failed to login with SFDX Authentication URL")?,
				CredentialsType::ClientCredentials | CredentialsType::Password => login_with_credential(credentials)
					.await
					.context("Failed to login with username and password")?,
				CredentialsType::None => credentials,
			},
			None => credentials,
		};

		Ok(Client {
			http_client,
			credentials,
			version: "v60.0".to_string(),
		})
	}

	/// Retrieves the instance URL as an optional string slice.
	///
	/// This function returns the value of `instance_url` if it is present,
	/// or `None` if the value is absent.
	///
	/// # Returns
	///
	/// - `Some(&str)` if `instance_url` contains a value.
	/// - `None` if `instance_url` is `None`.
	///
	/// # Examples
	///
	/// ```
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///     let endpoint = client.instance_url();
	///     Ok(())
	/// }
	/// ```
	pub fn instance_url(&self) -> Option<&str> {
		self.credentials.instance_url()
	}

	pub fn validated_base_url(&self) -> Result<String> {
		let instance_url = self.credentials.instance_url().context("Not logged in")?;
		Ok(format!("{}/services/data/", instance_url))
	}

	/// Returns the version of the current instance.
	///
	/// # Returns
	/// A string slice that holds the version information.
	///
	/// # Examples
	/// ```
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///     assert_eq!(client.version(), "v60.0");
	///     Ok(())
	/// }
	/// ```
	pub fn version(&self) -> &str {
		&self.version
	}

	/// Returns the bare version of the current instance. (without the 'v' prefix)
	///
	/// # Returns
	/// A string that holds the version information.
	///
	/// # Examples
	/// ```
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///     assert_eq!(client.version(), "v60.0");
	///     Ok(())
	/// }
	/// ```
	pub fn version_number(&self) -> Result<String> {
		let mut chars = self.version.chars();
		chars.next();
		Ok(chars.as_str().to_string()) // fixme might want to store just the base version number and then add the "v" when really needed...
	}

	/// Constructs the base path URL for the API, incorporating the instance URL and version.
	///
	/// # Returns
	///
	/// - `Ok(String)` containing the formatted base path URL if the `instance_url` is available.
	/// - `Err(Error::NotLoggedIn)` if the `instance_url` is not set.
	///
	/// # Errors
	///
	/// Returns an error of type `Error::NotLoggedIn` if the user is not logged in and the `instance_url`
	/// is unavailable.
	///
	/// # Example
	///
	/// ```rust
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///     match client.base_version_path() {
	///         Ok(base_path) => println!("Base path: {}", base_path),
	///         Err(e) => println!("Error: {}", e),
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # Note
	///
	/// This method assumes that `version` is a valid string and does not perform validation on it.
	pub fn base_version_path(&self) -> Result<String> {
		let instance_url = self.credentials.instance_url().context("Not logged in")?;
		Ok(format!("{}/services/data/{}", instance_url, self.version))
	}

	pub fn base_path(&self) -> Result<String> {
		let instance_url = self.credentials.instance_url().context("Not logged in")?;
		Ok(format!("{}/services/data/", instance_url))
	}

	pub fn soap_path(&self) -> Result<String> {
		let instance_url = self.credentials.instance_url().context("Not logged in")?;
		Ok(format!("{}/services/Soap/m/", instance_url))
	}

	pub fn soap_version_path(&self) -> Result<String> {
		let instance_url = self.credentials.instance_url().context("Not logged in")?;
		Ok(format!("{}/services/Soap/m/{}", instance_url, self.version_number()?))
	}

	/// Sets the version for the current instance.
	///
	/// This method updates the `version` field of the struct to the provided string.
	/// It consumes a string slice, converts it to a `String`, and assigns it to the
	/// instance's `version` field.
	///
	/// # Parameters
	/// - `version`: A string slice representing the new version to be set.
	///
	/// # Returns
	/// A mutable reference to the current instance, allowing for method chaining.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///     client.set_version("v65.0");
	///     Ok(())
	/// }
	/// ```
	pub fn set_version(&mut self, version: &str) -> &mut Self {
		self.version = version.to_string();
		self
	}

	pub fn get_user_id(&self) -> Option<&str> {
		self.credentials.user_id.as_deref()
	}

	/// Asynchronously ensures that the access token is refreshed if it has expired or
	/// if it cannot be parsed for comparison.
	///
	/// # Behavior
	/// - If `self.access_token` is `None`, this function does nothing and returns `Ok(self)`.
	/// - If the `issued_at` timestamp of the `access_token` cannot be parsed as a Unix
	///   timestamp (milliseconds since epoch), the function will attempt to refresh
	///   the access token immediately.
	/// - If the `issued_at` timestamp is successfully parsed and the corresponding access token
	///   is still valid (within 2 hours of being issued), no action is taken.
	/// - If the access token has expired (2 hours past the `issued_at` time), the function
	///   will attempt to refresh the token.
	///
	/// # Returns
	/// - `Ok(&mut Self)` if the access token is valid or successfully refreshed.
	/// - `Err(Error)` if there is an error during the refresh process.
	///
	/// # Logs
	/// - Logs an informational message if the `issued_at` timestamp cannot be parsed.
	/// - Logs an informational message if the access token has expired and a refresh is attempted.
	///
	/// # Errors
	/// Returns an `Error` if the refresh operation fails.
	///
	/// # Examples
	/// ```rust
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// async fn example() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     client.ensure_refresh().await?;
	///     Ok(())
	/// }
	/// ```
	pub async fn ensure_refresh(&mut self) -> Result<&mut Self> {
		if !self.credentials.has_access_token() {
			return Ok(self);
		}

		if self.credentials.is_access_token_expired() {
			log::info!("Access Token Expired, Refreshing.");
			self.credentials.set_access_token(Some(
				get_new_access_token(&self.credentials)
					.await
					.context("Failed to refresh access token")?,
			));
			Ok(self)
		} else {
			Ok(self)
		}
	}

	/// Asynchronously performs an HTTP GET request to a specified Apex REST endpoint using a full URI.
	///
	/// # Parameters
	/// - `uri`: A string slice representing the relative path or endpoint of the resource to be accessed.
	///
	/// # Returns
	/// - `Result<Response, Error>`:
	///   - On success, returns a `Response` containing the result of the GET request.
	///   - On failure, returns an `Error` indicating the reason for failure:
	///     - `Error::NotLoggedIn`: If the `instance_url` field is not available (user is not logged in).
	///     - `Error::ConfigError`: If the generated URL for the request is invalid.
	///
	/// # Description
	/// This function constructs the full URL for the requested resource by combining the base
	/// `instance_url` (from the current state) with the provided `uri` parameter. It validates
	/// the resulting URL and extracts query parameters for the request. The function then delegates
	/// the constructed path and parameters to the `rest_get` method, which executes the actual GET request.
	///
	/// The following steps are performed:
	/// 1. Combine the base `instance_url` and `uri` to generate the full resource URL.
	/// 2. Parse the URL to verify its validity and extract query parameters.
	/// 3. Convert query parameters into suitable format for internal processing.
	/// 4. Extract the URL path and invoke `rest_get` with the path and query parameters.
	///
	/// # Errors
	/// - Returns `Error::NotLoggedIn` if no `instance_url` is available in the current context.
	/// - Returns `Error::ConfigError` if URL parsing fails due to an invalid format.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///     match client.rest_get_fulluri("my/resource/endpoint").await {
	///         Ok(response) =>  println!("Response: {:?}", response),
	///         Err(e) => println!("Error: {:?}", e),
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # Notes
	/// The function assumes that the `self.rest_get` method is implemented to handle the GET request
	/// based on the extracted path and parameters.
	pub async fn rest_get_fulluri(&mut self, uri: &str) -> Result<Response> {
		let resource_url = format!(
			"{}/services/apexrest/{}",
			self.credentials.instance_url().context("Not logged in")?,
			uri
		);
		let parsed = Url::parse(&resource_url).context(format!("Invalid URL: {}", resource_url))?;
		// Some ownership absurdity for string refs accessed through iterators with collect
		let hash_query: HashMap<_, _> = parsed.query_pairs().into_owned().collect();
		let params_string: Vec<(String, String)> = hash_query
			.keys()
			.map(|k| (String::from(k), String::from(&hash_query[k])))
			.collect();
		let params: Vec<(&str, &str)> = params_string.iter().map(|(x, y)| (&x[..], &y[..])).collect();
		let path: String = parsed.path().to_string();
		self.rest_get(path, params).await
	}

	/// Asynchronously performs an HTTP GET request to a specified REST API endpoint.
	///
	/// # Arguments
	///
	/// * `path` - A `String` representing the relative path of the endpoint to be queried on the server.
	/// * `params` - A `Vec` of key-value tuples (`&str`, `&str`) representing query parameters to be appended to the URL.
	///
	/// # Returns
	///
	/// * `Result<Response, Error>` -
	///   - On success: Returns an `Ok(Response)` containing the HTTP response.
	///   - On failure: Returns an `Err(Error)` indicating the reason for failure.
	///
	/// # Errors
	///
	/// This function may return errors in the following scenarios:
	/// * `Error::NotLoggedIn` - If the `instance_url` is not set, indicating the user is not logged in.
	/// * Errors originating from HTTP client operations, such as issues while creating headers, sending the request, or ensuring token refresh.
	///
	/// # Example
	///
	/// ```rust
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///     let path = String::from("/api/v1/data");
	///     let params = vec![("key", "value"), ("filter", "recent")];
	///     let response = client.rest_get(path, params).await;
	///
	///     match response {
	///         Ok(res) => {
	///             println!("Response received: {:?}", res);
	///         }
	///         Err(e) => {
	///             eprintln!("Error occurred: {:?}", e);
	///         }
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # Notes
	///
	/// This function ensures that the client's authentication tokens are refreshed before sending the request.
	/// It constructs the full URL by concatenating the `instance_url` and the provided `path`. Query parameters are appended to the URL as needed.
	///
	/// This function assumes you have an internal configuration for `http_client` and a means to handle HTTP headers through `create_header`.
	pub async fn rest_get(&mut self, path: String, params: Vec<(&str, &str)>) -> Result<Response> {
		self.ensure_refresh().await?;

		let url = format!("{}{}", self.credentials.instance_url().context("Not logged in")?, path);
		let res = self
			.http_client
			.get(url.as_str())
			.headers(self.create_header(vec![])?)
			.query(&params)
			.send()
			.await?;
		Ok(res)
	}

	/// Sends a POST request to the specified REST API endpoint with the provided parameters.
	///
	/// # Type Parameters
	/// * `T`: The type of the request parameters. Must implement the `Serialize` trait.
	///
	/// # Parameters
	/// * `path`: A `String` specifying the API endpoint relative to the base instance URL.
	/// * `params`: An object of type `T` containing the data to be serialized as JSON and sent
	///   in the POST request body.
	///
	/// # Returns
	/// Returns a `Result<Response, Error>` that represents either:
	/// * `Ok(Response)` - The successful HTTP response.
	/// * `Err(Error)` - An error encountered during the process (e.g., not logged in,
	///   serialization issues, network errors, etc.).
	///
	/// # Errors
	/// This function can return the following errors:
	/// * `Error::NotLoggedIn` - If the client is not logged in or the instance URL is missing.
	/// * Other errors may be returned due to HTTP request failures or issues during request setup.
	///
	/// # Behavior
	/// 1. Ensures the client session is refreshed by calling `ensure_refresh()` asynchronously.
	/// 2. Constructs the full URL by appending the given `path` to the instance URL.
	/// 3. Sets the appropriate headers using `create_header`.
	/// 4. Serializes the `params` into JSON.
	/// 5. Sends an asynchronous POST request to the constructed URL.
	/// 6. Returns the received HTTP response upon success.
	///
	/// # Examples
	/// ```rust
	/// use serde::Serialize;
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// #[derive(Serialize)]
	/// struct Params {
	///     key: String,
	///     value: i32,
	/// }
	///
	/// async fn example() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication...
	///     let params = Params {
	///         key: "example".into(),
	///         value: 42,
	///     };
	///     let response = client
	///         .rest_post("/api/endpoint".to_string(), params)
	///         .await?;
	///
	///     println!("Response: {:?}", response);
	///     Ok(())
	/// }
	/// ```
	pub async fn rest_post<T: Serialize>(&mut self, path: String, params: T) -> Result<Response> {
		self.ensure_refresh().await?;

		let url = format!("{}{}", self.credentials.instance_url().context("Not logged in")?, path);
		let res = self
			.http_client
			.post(url)
			.headers(self.create_header(vec![])?)
			.json(&params)
			.send()
			.await?;
		Ok(res)
	}

	/// Sends a PATCH request to the specified REST API endpoint with the given parameters.
	///
	/// # Type Parameters
	/// - `T`: The type of the parameters to be serialized into the request body. It must implement the `Serialize` trait.
	///
	/// # Arguments
	/// - `path`: A `String` representing the relative path of the API endpoint to send the request to.
	/// - `params`: An object of type `T` containing the parameters to be serialized into the JSON body of the request.
	///
	/// # Returns
	/// - `Ok(Response)`: On a successful request, returns the HTTP response of type `Response`.
	/// - `Err(Error)`: Returns an error if:
	///     - The instance is not properly set up or the user is not logged in (`Error::NotLoggedIn`).
	///     - There is an issue with refreshing the instance or creating the request headers.
	///     - The HTTP request fails for some other reason (e.g., a network error).
	///
	/// # Errors
	/// This method may return an error in the following cases:
	/// - The user is not logged in (`Error::NotLoggedIn`) and the `instance_url` is unavailable.
	/// - If the `ensure_refresh` internal function fails (e.g., unable to refresh authentication tokens).
	/// - If there is an error serializing the parameters or constructing the HTTP headers.
	/// - If the HTTP request fails during transmission or response handling.
	///
	/// # Examples
	/// ```rust
	/// use serde::Serialize;
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// #[derive(Serialize)]
	/// struct UpdateParams {
	///     field: String,
	///     value: String,
	/// }
	///
	/// async fn example() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication...
	///     let params = UpdateParams {
	///         field: "example_field".to_string(),
	///         value: "new_value".to_string(),
	///     };
	///
	///     let response = client
	///         .rest_patch("/api/endpoint".to_string(), params)
	///         .await?;
	///
	///     println!("Response: {:?}", response);
	///     Ok(())
	/// }
	/// ```
	pub async fn rest_patch<T: Serialize>(&mut self, path: String, params: T) -> Result<Response> {
		self.ensure_refresh().await?;

		let url = format!("{}{}", self.credentials.instance_url().context("Not logged in")?, path);
		let res = self
			.http_client
			.patch(url.as_str())
			.headers(self.create_header(vec![])?)
			.json(&params)
			.send()
			.await?;
		Ok(res)
	}

	/// Sends an HTTP PUT request to the specified REST API endpoint with the given parameters in JSON format.
	///
	/// This asynchronous function constructs the full URL by appending the provided `path` to the
	/// instance URL, ensures the client is authenticated (by refreshing the session if necessary),
	/// and makes the PUT request using the provided parameters. The function expects the parameters
	/// to be serializable as JSON and attaches custom headers to the request.
	///
	/// # Type Parameters
	/// * `T`: A type that implements the [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) trait, representing
	///   the data to be serialized into JSON and included in the request body.
	///
	/// # Parameters
	/// * `path`: A [`String`](https://doc.rust-lang.org/std/string/struct.String.html) specifying the REST API endpoint's relative path.
	/// * `params`: A serializable object of type `T` that represents the payload to be included in the PUT request.
	///
	/// # Returns
	/// * `Result<Response, Error>`: On success, this function returns a successful [`Response`](https://docs.rs/reqwest/latest/reqwest/struct.Response.html) from the PUT request.
	///   On failure, it returns an [`Error`](https://docs.rs/reqwest/latest/reqwest/enum.Error.html) or custom `Error` defined by the client.
	///
	/// # Errors
	/// * Returns `Error::NotLoggedIn` if the client is not logged in and the `instance_url` is not available.
	/// * Propagates errors encountered while refreshing the session via `self.ensure_refresh()`.
	/// * Propagates errors from invalid header creation, serializing the payload, sending the HTTP request, or other
	///   networking issues encountered by the HTTP client.
	///
	/// # Examples
	/// ```rust
	/// use serde::Serialize;
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// #[derive(Serialize)]
	/// struct UpdateData {
	///     key: String,
	///     value: String,
	/// }
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()>  {
	///     let mut client= Client::new(Credentials::new()).await?;
	///
	///     let data = UpdateData {
	///         key: "example_key".to_string(),
	///         value: "example_value".to_string(),
	///     };
	///
	///     match client.rest_put("/api/resource".to_string(), data).await {
	///         Ok(response) => {
	///             println!("Request succeeded with status: {}", response.status());
	///         }
	///         Err(e) => {
	///             eprintln!("Request failed: {}", e);
	///         }
	///     }
	///     Ok(())
	/// }
	/// ```
	pub async fn rest_put<T: Serialize>(&mut self, path: String, params: T) -> Result<Response> {
		self.ensure_refresh().await?;

		let url = format!("{}{}", self.credentials.instance_url().context("Not logged in")?, path);
		let res = self
			.http_client
			.put(url.as_str())
			.headers(self.create_header(vec![])?)
			.json(&params)
			.send()
			.await?;
		Ok(res)
	}

	/// Sends an asynchronous HTTP DELETE request to a specified path.
	///
	/// # Parameters
	/// - `path` (`String`): The path to which the DELETE request will be sent. This path will be appended to the instance URL.
	///
	/// # Returns
	/// - `Result<Response, Error>`:
	///     - On success, returns an `Ok(Response)` representing the HTTP response from the server.
	///     - On failure, returns an `Err(Error)` detailing the error encountered.
	///
	/// # Errors
	/// - This function will return an error in the following scenarios:
	///   - If the client is not logged in (`Error::NotLoggedIn`).
	///   - If refreshing the authentication token fails.
	///   - If there is an issue creating the HTTP headers.
	///   - If the HTTP request fails during execution.
	///
	/// # Async Behavior
	/// - This function is asynchronous and should be awaited.
	/// - Before sending the DELETE request, it ensures that the authentication state is up-to-date by calling `self.ensure_refresh()`.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///     let response = client.rest_delete("/resource/123".to_string()).await;
	///
	///     match response {
	///         Ok(res) => {
	///             println!("DELETE request successful: {}", res.status());
	///         }
	///         Err(err) => {
	///             eprintln!("Error occurred: {}", err);
	///         }
	///     }
	///     Ok(())
	/// }
	/// ```
	pub async fn rest_delete(&mut self, path: String) -> Result<Response> {
		self.ensure_refresh().await?;

		let url = format!("{}{}", self.credentials.instance_url().context("Not logged in")?, path);
		let res = self
			.http_client
			.delete(url.as_str())
			.headers(self.create_header(vec![])?)
			.send()
			.await?;
		Ok(res)
	}

	/// Sends an asynchronous HTTP GET request to the specified URL with the provided query parameters.
	///
	/// This function ensures that any necessary pre-request setup (such as refreshing tokens or credentials)
	/// is performed before sending the request. It then constructs the request with appropriate headers and
	/// query parameters, uses the underlying HTTP client to send the request, and returns the response or an error.
	///
	/// # Arguments
	///
	/// * `url` - A `String` representing the target URL for the GET request.
	/// * `params` - A `Vec` of tuples where each tuple contains a key-value pair (`String`, `String`) representing
	///              query parameters to append to the URL.
	///
	/// # Returns
	///
	/// Returns a `Result` containing:
	/// - `Response`: The HTTP response object if the request is successful.
	/// - `Error`: An error if an issue occurs during the process of refreshing credentials,
	///   constructing the request, or sending it.
	///
	/// # Errors
	///
	/// This function can return an `Error` due to:
	/// - Failure during the token/credential refresh process in `self.ensure_refresh()`.
	/// - Issues with building headers in `self.create_header`.
	/// - Errors originating from the underlying HTTP client's `send` function (e.g., connectivity issues).
	///
	/// # Example
	/// ```rust
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///     let response = client.get(
	///         "https://api.example.com/items".into(),
	///         vec![("key".into(), "value".into())],
	///         vec![]
	///     ).await;
	///     match response {
	///         Ok(res) => {
	///             println!("Response: {:?}", res);
	///         },
	///         Err(err) => {
	///             eprintln!("Error: {:?}", err);
	///         }
	///     }
	///     Ok(())
	/// }
	/// ```
	pub async fn get(
		&mut self,
		url: String,
		params: Vec<(String, String)>,
		headers: Vec<(String, String)>,
	) -> Result<Response> {
		self.ensure_refresh().await?;

		debug!("GET: {} params: {:?} headers: {:?}", url, params, headers);

		let res = self
			.http_client
			.get(url.as_str())
			.headers(self.create_header(headers)?)
			.query(&params)
			.send()
			.await?;
		Ok(res)
	}

	/// Sends a GET request to the specified URL with optional additional headers,
	/// ensuring that necessary token or session refresh procedures are completed before the request.
	///
	/// # Arguments
	///
	/// * `url` - A string slice that holds the URL to which the GET request will be sent.
	/// * `additional_headers` - A vector of key-value pairs (`String`, `String`) representing
	///   additional headers to include in the request. Note that the "Accept" header is removed
	///   from the headers before the request is sent.
	///
	/// # Returns
	///
	/// * `Result<Response, Error>` - On success, returns a `Response` object representing the
	///   HTTP response. On failure, returns an `Error` indicating what went wrong.
	///
	/// # Errors
	///
	/// This function returns an error in the following scenarios:
	///
	/// * If `self.ensure_refresh()` fails during the session or token refresh process.
	/// * If `self.create_header()` fails to create the HTTP headers from `additional_headers`.
	/// * If the HTTP client encounters an issue while sending the GET request.
	///
	/// # Example
	///
	/// ```rust
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///     let  url = "https://example.com/api/resource";
	///     let  headers = vec![("Authorization".to_string(), "Bearer token".to_string())];
	///     match client.get_raw(url, headers).await {
	///         Ok(response) => println!("Response status: {}", response.status()),
	///         Err(err) => println!("Error: {}", err),
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # Notes
	///
	/// This method removes the "Accept" header (if any) from the list of headers before
	/// sending the request.
	pub async fn get_raw(&mut self, url: &str, additional_headers: Vec<(String, String)>) -> Result<Response> {
		self.ensure_refresh().await?;

		debug!("GET: {} headers: {:?}", url, additional_headers);

		let mut headers = self.create_header(additional_headers)?;
		headers.remove("Accept");
		let res = self.http_client.get(url).headers(headers).send().await?;
		Ok(res)
	}

	/// Asynchronously sends an HTTP POST request to the specified URL with the given parameters and headers.
	///
	/// This function ensures the client is refreshed before making the request.
	/// It constructs the POST request by combining the given URL, serialization of parameters, and custom headers,
	/// and uses the `reqwest` HTTP client to send the request.
	///
	/// # Type Parameters
	///
	/// * `T`: A type that implements the `Serialize` trait, representing the body of the POST request.
	///
	/// # Parameters
	///
	/// * `url` - A `String` representing the URL endpoint for the POST request.
	/// * `params` - A serializable type (`T`) used as the JSON body of the POST request.
	/// * `headers` - A vector of tuples where each tuple contains a header name and its corresponding value (`Vec<(String, String)>`).
	///
	/// # Returns
	///
	/// Returns a `Result`:
	/// - `Ok(Response)` on success, where `Response` is the HTTP response returned by the request.
	/// - `Err(Error)` if an error occurs during the request, such as serialization issues, header creation errors, or HTTP client errors.
	///
	/// # Errors
	///
	/// This function may return an error in the following circumstances:
	/// * If `self.ensure_refresh()` fails.
	/// * If `self.create_header(headers)` fails to construct the headers.
	/// * If the HTTP client encounters an issue while sending the request.
	///
	/// # Examples
	///
	/// ```rust
	/// use rustsf::{Client, Credentials, BulkApiV2, DefSObject};
	/// use serde::{Deserialize, Serialize};
	/// use anyhow::Result;
	///
	/// #[DefSObject(sobject_type = "Account", fields="system,type,name")]
	/// struct Account {}
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     use rustsf::primary_types::SObject;
	/// let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///
	///     let headers = vec![("Authorization".to_string(), "Bearer token".to_string())];
	///     let url = "https://www.salesforce.com/api/v60.0".to_string();
	///     let mut acc = Account::new();
	///     acc.set_id(Some("001xx000003DGbX"));
	///     acc.name = Some("Test".to_string());
	///
	///     let response = client.post(url, acc, headers).await;
	///     match response {
	///         Ok(res) => println!("Response: {:?}", res),
	///         Err(err) => eprintln!("Error: {:?}", err),
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// Note: Ensure that the given `url` is valid and accessible and that proper error handling is implemented
	/// for production use.
	pub async fn post<T: Serialize + Debug>(
		&mut self,
		url: String,
		params: T,
		headers: Vec<(String, String)>,
	) -> Result<Response> {
		self.ensure_refresh().await?;

		debug!("POST: {} params: {:?} headers: {:?}", url, params, headers);

		let res = self
			.http_client
			.post(url)
			.headers(self.create_header(headers)?)
			.json(&params)
			.send()
			.await?;
		Ok(res)
	}

	pub async fn post_soap(&mut self, action: &str, body: String) -> Result<Response> {
		let url = self.soap_version_path()?;
		debug!("Soap Metadata API '{}' request: POST {} : {}", action, url, body);

		Ok(self
			.http_client
			.post(url)
			.header("Content-Type", "text/xml")
			.header("SOAPAction", format!("'{}'", action))
			.body(body)
			.send()
			.await?)
	}

	/// Asynchronously sends an HTTP POST multipart equest to the specified URL with the given parameters and headers.
	///
	/// This function ensures the client is refreshed before making the request.
	/// It constructs the POST request by combining the given URL, serialization of parameters, and custom headers,
	/// and uses the `reqwest` HTTP client to send the request.
	///
	/// # Type Parameters
	///
	/// * `T`: A type that implements the `Serialize` trait, representing the body of the POST request.
	///
	/// # Parameters
	///
	/// * `url` - A `String` representing the URL endpoint for the POST request.
	/// * `headers` - A vector of tuples where each tuple contains a header name and its corresponding value (`Vec<(String, String)>`).
	/// * `from` - A multipart form to include in the request.
	///
	/// # Returns
	///
	/// Returns a `Result`:
	/// - `Ok(Response)` on success, where `Response` is the HTTP response returned by the request.
	/// - `Err(Error)` if an error occurs during the request, such as serialization issues, header creation errors, or HTTP client errors.
	///
	/// Note: Ensure that the given `url` is valid and accessible and that proper error handling is implemented
	/// for production use.
	#[cfg(feature = "metadata-api")]
	pub async fn post_multipart(
		&mut self,
		url: String,
		headers: Vec<(String, String)>,
		form: Form,
	) -> Result<Response> {
		println!("url {:?}", url);

		let request = self
			.http_client
			.post(&url)
			.multipart(form)
			.headers(self.create_header(headers)?)
			.build()?;

		println!("Request URL: {}", request.url());
		println!("Request Method: {}", request.method());

		// Log headers
		for (name, value) in request.headers() {
			println!("Header {}: {:?}", name, value);
		}

		// Log body size (reqwest hides the exact body bytes if it's a stream/json,
		// but you can check if a body exists)
		if let Some(body) = request.body() {
			println!("Body: {:?}", body.as_bytes());
		}

		let response = self.http_client.execute(request).await?;

		Ok(response)
	}

	/// Asynchronously sends an HTTP POST request to the specified URL with the given parameters and headers.
	///
	/// This function ensures the client is refreshed before making the request.
	/// It constructs the POST request by combining the given URL, serialization of parameters, and custom headers,
	/// and uses the `reqwest` HTTP client to send the request.
	///
	/// # Type Parameters
	///
	/// * `T`: A type that implements the `Serialize` trait, representing the body of the POST request.
	///
	/// # Parameters
	///
	/// * `url` - A `String` representing the URL endpoint for the POST request.
	/// * `params` - A serializable type (`T`) used as the JSON body of the POST request.
	/// * `headers` - A vector of tuples where each tuple contains a header name and its corresponding value (`Vec<(String, String)>`).
	///
	/// # Returns
	///
	/// Returns a `Result`:
	/// - `Ok(Response)` on success, where `Response` is the HTTP response returned by the request.
	/// - `Err(Error)` if an error occurs during the request, such as serialization issues, header creation errors, or HTTP client errors.
	///
	/// # Errors
	///
	/// This function may return an error in the following circumstances:
	/// * If `self.ensure_refresh()` fails.
	/// * If `self.create_header(headers)` fails to construct the headers.
	/// * If the HTTP client encounters an issue while sending the request.
	///
	/// # Examples
	///
	/// ```rust
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///     let headers = vec![("Authorization".to_string(), "Bearer token".to_string())];
	///     let url = "https://example.com/api/accounts".to_string();
	///     match client.post_raw_buffer(url, vec![], headers).await {
	///         Ok(res) => println!("Response: {:?}", res),
	///         Err(err) => println!("Error: {:?}", err),
	///     }
	///     Ok(())
	/// }
	/// ```
	pub async fn post_raw_buffer(
		&mut self,
		url: String,
		body: Vec<u8>,
		headers: Vec<(String, String)>,
	) -> Result<Response> {
		self.ensure_refresh().await?;

		debug!("POST: {} body: {:?} headers: {:?}", url, body, headers);

		let res = self
			.http_client
			.post(url)
			.headers(self.create_header(headers)?)
			.body(body)
			.send()
			.await?;
		Ok(res)
	}

	/// Sends an HTTP PUT request to the specified URL with the provided content.
	///
	/// This method ensures that necessary pre-request refresh actions are performed,
	/// sets the proper headers (`Content-Type` as `text/csv` and `Accept` as `application/json`),
	/// and sends the supplied data as the request body.
	///
	/// # Arguments
	///
	/// * `url` - A `String` containing the target URL for the PUT request.
	/// * `buffer` - A `Vec<u8>` containing the binary data to be sent as the request body.
	///
	/// # Returns
	///
	/// This method returns a `Result`:
	/// * `Ok(Response)` - The HTTP response returned by the server.
	/// * `Err(Error)` - An error if the refresh action, header creation,
	///   or the HTTP request itself fails.
	///
	/// # Errors
	///
	/// This function can return an error in the following cases:
	/// * If the `ensure_refresh` method fails to complete successfully.
	/// * If header creation fails to generate a valid set of headers.
	/// * If the `http_client.put` request encounters an issue during sending or receiving.
	///
	/// # Examples
	///
	/// ```rust
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///     let url = "https://example.com/resource".to_string();
	///     let data = b"column1,column2\nvalue1,value2".to_vec();
	///
	///     match client.put(url, data).await {
	///         Ok(response) => println!("PUT request succeeded with status: {}", response.status()),
	///         Err(err) => eprintln!("PUT request failed: {}", err),
	///     }
	///     Ok(())
	/// }
	/// ```
	pub async fn put(&mut self, url: String, buffer: Vec<u8>) -> Result<Response> {
		self.ensure_refresh().await?;

		debug!("PUT: {} body: {:?}", url, buffer);

		let mut headers = self.create_header(vec![])?;
		headers.insert("Content-Type", HeaderValue::from_static("text/csv"));
		headers.insert("Accept", HeaderValue::from_static("application/json"));
		let res = self
			.http_client
			.put(url.as_str())
			.headers(headers)
			.body(buffer)
			.send()
			.await?;
		Ok(res)
	}

	/// Sends an asynchronous HTTP PATCH request to the specified URL with the provided parameters.
	///
	/// # Type Parameters
	/// - `T`: A type that implements the `Serialize` trait, representing the body of the request.
	///
	/// # Parameters
	/// - `url`: A `String` containing the target URL for the PATCH request.
	/// - `params`: A value of type `T`, used as the serialized JSON body of the PATCH request.
	///
	/// # Returns
	/// - `Result<Response, Error>`:
	///   - `Ok(Response)`: If the request is successful, returns the response from the server.
	///   - `Err(Error)`: If an error occurs during the request (e.g., problems in refreshing state,
	///     creating headers, JSON serialization, or network issues), returns the corresponding error.
	///
	/// # Errors
	/// This function returns an error if:
	/// - `ensure_refresh` fails to refresh the client state.
	/// - An error occurs while creating the headers.
	/// - An error occurs in serializing the `params`.
	/// - The HTTP client fails to send the request or receives an error response.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{Client, Credentials, DefSObject};
	/// use serde::Serialize;
	/// use anyhow::Result;
	///
	/// #[DefSObject(sobject_type = "Account", fields="name")]
	/// struct Account { }
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///     let url = String::from("https://example.com/resource");
	///     let mut acc = Account::new();
	///     acc.id = Some(String::from("001D000000IqhSLIAZ"));
	///     acc.name = Some(String::from("Sample Account"));
	///
	///     match client.patch::<Account>(url, acc).await {
	///         Ok(response) => println!("Request succeeded: {:?}", response),
	///         Err(err) => eprintln!("Request failed: {}", err),
	///     }
	///     Ok(())
	/// }
	/// ```
	pub async fn patch<T: Serialize + Debug>(&mut self, url: String, params: T) -> Result<Response> {
		self.ensure_refresh().await?;

		debug!("PATCH: {} params: {:?}", url, params);

		let res = self
			.http_client
			.patch(url.as_str())
			.headers(self.create_header(vec![])?)
			.json(&params)
			.send()
			.await?;
		Ok(res)
	}

	///
	/// Sends an HTTP DELETE request to the specified URL and returns the response.
	///
	/// This function performs the following operations:
	/// 1. Ensures that any necessary refresh operations are completed by calling `ensure_refresh`.
	/// 2. Constructs a DELETE request to the given URL using the internal HTTP client.
	/// 3. Adds the necessary headers to the request (created via `create_header`).
	/// 4. Sends the request asynchronously and waits for the response.
	///
	/// # Parameters
	/// - `url`: A `String` representing the URL to which the DELETE request will be sent.
	///
	/// # Returns
	/// - `Ok(Response)`: An HTTP response if the request is successful.
	/// - `Err(Error)`: An error if any part of the process fails, such as constructing headers,
	///   sending the request, or encountering issues during the refresh process.
	///
	/// # Errors
	/// This function will return an `Error` in the following cases:
	/// - If `ensure_refresh` fails.
	/// - If there is an issue with creating the headers.
	/// - If sending the DELETE request fails (e.g., network issues or server errors).
	///
	/// # Examples
	/// ```
	/// use rustsf::{Client, Credentials};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///
	///     let url = "https://api.example.com/resource".to_string();
	///     match client.delete(url).await {
	///         Ok(response) => println!("DELETE request succeeded with status: {}", response.status()),
	///         Err(err) => eprintln!("DELETE request failed: {}", err),
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # Note
	/// Ensure that the client is properly initialized and authenticated before calling this method.
	///
	pub async fn delete(&mut self, url: String) -> Result<Response> {
		self.ensure_refresh().await?;

		debug!("DELETE: {}", url);

		let res = self
			.http_client
			.delete(url.as_str())
			.headers(self.create_header(vec![])?)
			.send()
			.await?;
		Ok(res)
	}

	/// Generates a `HeaderMap` containing default headers, authorization, and additional custom headers.
	///
	/// # Arguments
	///
	/// * `additional_headers` - A list of key-value pairs representing additional headers
	///   to be added to the request. Each key is the header name and each value is the corresponding header value.
	///
	/// # Returns
	///
	/// * `Ok(HeaderMap)` - Returns the constructed `HeaderMap` with all headers included.
	/// * `Err(Error)` - Returns an error in the following scenarios:
	///   - If the access token is not found (`Error::NotLoggedIn`).
	///   - If a provided header name is invalid (`Error::ConfigError`).
	///   - If a provided header value cannot be parsed.
	///
	/// # Behavior
	///
	/// - Authenticates the request by including an `Authorization` header with a Bearer token.
	///   The token is retrieved from the `access_token` field of the struct, and an error is
	///   returned if it is not available.
	/// - Sets a default `Accept: application/json` header.
	/// - Iterates through the `additional_headers` list and adds the headers
	///   to the `HeaderMap`. If a header name already exists, the existing value is replaced.
	/// - All header names are validated for correctness, and invalid header names or values will result in an error.
	///
	/// # Errors
	///
	/// - `Error::NotLoggedIn` occurs when there is no access token available.
	/// - `Error::ConfigError` occurs if the custom header name is invalid or unparseable.
	/// - Errors are propagated if the header value conversion fails.
	///
	fn create_header(&self, additional_headers: Vec<(String, String)>) -> Result<HeaderMap> {
		let mut headers = HeaderMap::new();
		let auth_value = format!(
			"Bearer {}",
			self.credentials.access_token().context("Not logged in")?.value
		);
		headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_value)?);

		//Default header
		headers.insert("Accept", HeaderValue::from_static("application/json"));

		for (key, value) in additional_headers {
			let header_name: HeaderName = key.parse().context(format!("Invalid header name: {}", key))?;

			let header_value = HeaderValue::from_str(&value)?;

			//delete duplicates
			headers.remove(&header_name);

			headers.insert(header_name, header_value);
		}

		Ok(headers)
	}
}

/// Logs in using the provided username and password credentials,
/// retrieves an access token from the OAuth2 token endpoint,
/// and updates the internal client state.
///
/// # Arguments
///
/// - `username`: A string slice containing the user's username.
/// - `password`: A string slice containing the user's password.
///
/// # Returns
///
/// Returns:
/// - `Ok(&mut Self)` if the login and token retrieval are successful, allowing method chaining.
/// - `Err(Error)` if there are any configuration issues, HTTP request errors, or unsuccessful responses.
///
/// # Errors
///
/// The function can return the following errors:
/// - `ConfigError`: If `client_id` or `client_secret` is not configured.
/// - `HttpError`: If there is an issue with the HTTP request, such as network failure.
/// - `TokenError`: If the server returns an unsuccessful response, e.g., invalid credentials.
///
/// # Example
///
/// ```rust
/// use rustsf::{Client, Credentials};
/// use anyhow::Result;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let mut credentials = Credentials::new();
///     credentials.set_client_id("example_client_id");
///     credentials.set_client_secret("example_client_secret");
///
///     let mut client= Client::new(credentials).await;
///
///     match client {
///         Ok(client) => println!("Login successful!"),
///         Err(e) => eprintln!("Login failed: {:?}", e),
///     }
///     Ok(())
/// }
/// ```
///
/// # Behavior
///
/// - Constructs a token request URL based on the configured `login_endpoint`.
/// - Validates the presence of `client_id` and `client_secret` configurations.
/// - Sends an HTTP POST request with the required parameters to fetch an access token.
/// - Upon a successful response:
///     - Updates the internal client state with the access token, token type, issue timestamp, and instance URL.
/// - On error, the function returns a descriptive `Error` for troubleshooting.
///
/// # Notes
///
/// - Ensure that the `client_id` and `client_secret` are set before calling this method.
/// - The function uses the `reqwest` library for HTTP requests and assumes the `http_client` is properly initialized.
pub async fn login_with_credential(mut credentials: Credentials) -> Result<Credentials> {
	let token_url = format!("{}/services/oauth2/token", &credentials.login_endpoint());
	let client_id = credentials
		.client_id()
		.context("Client ID is not configured")?
		.to_string();
	let client_secret = credentials
		.client_secret()
		.context("Client Secret is not configured")?
		.to_string();
	let username = credentials.username().unwrap_or("").to_string();
	let password = credentials.password().unwrap_or("").to_string();

	let params = [
		("grant_type", "password"),
		("client_id", client_id.as_str()),
		("client_secret", client_secret.as_str()),
		("username", username.as_str()),
		("password", password.as_str()),
	];
	let http_client = reqwest::Client::new();
	let res = http_client
		.post(token_url.as_str())
		.form(&params)
		.send()
		.await
		.context("Failed to get token")?;

	if !(res.status().is_success()) {
		let error_response = res.text().await?;
		return Err(anyhow!("Failed to get token: {}", error_response));
	}

	let response: TokenResponse = res.json().await.context("Failed to parse token response")?;
	let token_type = response.token_type.unwrap_or_default();
	credentials.set_access_token(Some(AccessToken::new(
		response.access_token,
		response.issued_at,
		token_type,
	)));
	credentials.set_instance_url(&response.instance_url);
	Ok(credentials)
}

async fn login_with_sfdx_auth_url(mut credentials: Credentials) -> Result<Credentials> {
	let response = get_refresh_token_response(&credentials).await?;

	if credentials.user_id.is_none() || credentials.organisation_id.is_none() {
		match extract_organisation_and_user_id(response.id.as_str()) {
			Some((org_id, user_id)) => {
				credentials.set_organisation_id(&org_id);
				credentials.set_user_id(&user_id);
			}
			None => {
				log::warn!("Could not extract organisation and user ID from token response.");
			}
		}
	}

	credentials.set_instance_url(&response.instance_url);
	credentials.access_token = Some(AccessToken::from_token_response(response));
	Ok(credentials)
}

async fn get_refresh_token_response(credentials: &Credentials) -> Result<TokenResponse> {
	let url = format!("{}/services/oauth2/token", credentials.login_endpoint());
	let params = [
		("grant_type", "refresh_token"),
		("client_id", credentials.client_id().context("Missing client id")?),
		(
			"refresh_token",
			credentials.refresh_token().context("Missing refresh token")?,
		),
	];

	let http_client = reqwest::Client::new();
	let res = http_client
		.post(url)
		.form(&params)
		.send()
		.await
		.context("Failed to refresh access token")?;

	if !res.status().is_success() {
		let error_response = res
			.text()
			.await
			.context("Failed to parse refresh token error response")?;
		return Err(anyhow!("Failed to get refresh token: {}", error_response));
	}

	Ok(res.json().await.context("Failed to parse refresh token response")?)
}

async fn get_new_access_token(credentials: &Credentials) -> Result<AccessToken> {
	let response = get_refresh_token_response(&credentials).await?;

	Ok(AccessToken::from_token_response(response))
}

fn extract_organisation_and_user_id(url: &str) -> Option<(String, String)> {
	let path = url.split_once("/id/")?.1;
	let mut parts = path.split('/');

	let organisation_id = parts.next()?.trim();
	let user_id = parts.next()?.trim();

	if organisation_id.is_empty() || user_id.is_empty() {
		return None;
	}

	Some((organisation_id.to_string(), user_id.to_string()))
}

#[cfg(test)]
mod tests {
	use super::*;
	use mockito::Server;
	use serde_json::json;

	async fn create_test_client(server_url: &str) -> Client {
		let mut credentials = Credentials::new();
		credentials.set_instance_url(server_url);
		credentials.set_access_token(Some(AccessToken::new(
			"test_token".to_string(),
			"9999999999000".to_string(),
			"Bearer".to_string(),
		)));

		Client::new(credentials).await.unwrap()
	}

	// --- Setters and getters ---
	#[tokio::test]
	async fn test_set_version() {
		let mut client = Client::new(Credentials::new()).await.unwrap();
		client.set_version("v55.0");
		assert_eq!(client.version, "v55.0");
	}

	// --- create_header ---

	#[tokio::test]
	async fn test_create_header_with_token() {
		let mut credentials = Credentials::new();
		credentials.set_access_token(Some(AccessToken::new(
			"mytoken".to_string(),
			"".to_string(),
			"Bearer".to_string(),
		)));
		let client = Client::new(credentials).await.unwrap();

		let headers = client.create_header(vec![]).unwrap();
		assert_eq!(headers.get("Authorization").unwrap(), "Bearer mytoken");
		assert_eq!(headers.get("Accept").unwrap(), "application/json");
	}

	#[tokio::test]
	async fn test_create_header_with_additional_headers() {
		let mut credentials = Credentials::new();
		credentials.set_access_token(Some(AccessToken::new(
			"mytoken".to_string(),
			"".to_string(),
			"Bearer".to_string(),
		)));
		let client = Client::new(credentials).await.unwrap();

		let headers = client
			.create_header(vec![
				("X-Custom".to_string(), "custom_value".to_string()),
				("Accept".to_string(), "text/xml".to_string()),
			])
			.unwrap();

		assert_eq!(headers.get("X-Custom").unwrap(), "custom_value");
		// Accept should be overridden
		assert_eq!(headers.get("Accept").unwrap(), "text/xml");
	}

	// --- HTTP methods with mock server ---

	#[tokio::test]
	async fn test_get() {
		let mut server = Server::new_async().await;
		let mock = server
			.mock("GET", "/test")
			.match_query(mockito::Matcher::AllOf(vec![]))
			.with_status(200)
			.with_body("ok")
			.create_async()
			.await;

		let mut client = create_test_client(&server.url()).await;
		let res = client.get(format!("{}/test", server.url()), vec![], vec![]).await;
		assert!(res.is_ok());
		assert_eq!(res.unwrap().status(), 200);
		mock.assert_async().await;
	}

	#[tokio::test]
	async fn test_post() {
		let mut server = Server::new_async().await;
		let mock = server
			.mock("POST", "/test")
			.with_status(201)
			.with_body(r#"{"id":"123","success":true}"#)
			.create_async()
			.await;

		let mut client = create_test_client(&server.url()).await;
		let mut params = std::collections::HashMap::new();
		params.insert("Name", "Test");
		let res = client.post(format!("{}/test", server.url()), params, vec![]).await;
		assert!(res.is_ok());
		assert_eq!(res.unwrap().status(), 201);
		mock.assert_async().await;
	}

	#[tokio::test]
	async fn test_patch() {
		let mut server = Server::new_async().await;
		let mock = server.mock("PATCH", "/test").with_status(204).create_async().await;

		let mut client = create_test_client(&server.url()).await;
		let mut params = std::collections::HashMap::new();
		params.insert("Name", "Updated");
		let res = client.patch(format!("{}/test", server.url()), params).await;
		assert!(res.is_ok());
		assert_eq!(res.unwrap().status(), 204);
		mock.assert_async().await;
	}

	#[tokio::test]
	async fn test_delete() {
		let mut server = Server::new_async().await;
		let mock = server.mock("DELETE", "/test").with_status(204).create_async().await;

		let mut client = create_test_client(&server.url()).await;
		let res = client.delete(format!("{}/test", server.url())).await;
		assert!(res.is_ok());
		assert_eq!(res.unwrap().status(), 204);
		mock.assert_async().await;
	}

	#[tokio::test]
	async fn test_put() {
		let mut server = Server::new_async().await;
		let mock = server.mock("PUT", "/test").with_status(201).create_async().await;

		let mut client = create_test_client(&server.url()).await;
		let res = client.put(format!("{}/test", server.url()), b"csv,data".to_vec()).await;
		assert!(res.is_ok());
		assert_eq!(res.unwrap().status(), 201);
		mock.assert_async().await;
	}

	#[tokio::test]
	async fn test_post_raw_buffer() {
		let mut server = Server::new_async().await;
		let mock = server.mock("POST", "/test").with_status(200).create_async().await;

		let mut client = create_test_client(&server.url()).await;
		let res = client
			.post_raw_buffer(
				format!("{}/test", server.url()),
				b"raw data".to_vec(),
				vec![("Content-Type".to_string(), "text/csv".to_string())],
			)
			.await;
		assert!(res.is_ok());
		mock.assert_async().await;
	}

	#[tokio::test]
	async fn test_get_raw() {
		let mut server = Server::new_async().await;
		let mock = server
			.mock("GET", "/test")
			.with_status(200)
			.with_body("raw response")
			.create_async()
			.await;

		let mut client = create_test_client(&server.url()).await;
		let res = client.get_raw(&format!("{}/test", server.url()), vec![]).await;
		assert!(res.is_ok());
		mock.assert_async().await;
	}

	// --- REST methods ---

	#[tokio::test]
	async fn test_rest_get() {
		let mut server = Server::new_async().await;
		let mock = server
			.mock("GET", "/some/path")
			.with_status(200)
			.with_body("ok")
			.create_async()
			.await;

		let mut client = create_test_client(&server.url()).await;
		let res = client.rest_get("/some/path".to_string(), vec![]).await;
		assert!(res.is_ok());
		mock.assert_async().await;
	}

	#[tokio::test]
	async fn test_rest_post() {
		let mut server = Server::new_async().await;
		let mock = server.mock("POST", "/some/path").with_status(201).create_async().await;

		let mut client = create_test_client(&server.url()).await;
		let mut params = std::collections::HashMap::new();
		params.insert("key", "value");
		let res = client.rest_post("/some/path".to_string(), params).await;
		assert!(res.is_ok());
		mock.assert_async().await;
	}

	#[tokio::test]
	async fn test_rest_patch() {
		let mut server = Server::new_async().await;
		let mock = server.mock("PATCH", "/some/path").with_status(204).create_async().await;

		let mut client = create_test_client(&server.url()).await;
		let mut params = std::collections::HashMap::new();
		params.insert("key", "value");
		let res = client.rest_patch("/some/path".to_string(), params).await;
		assert!(res.is_ok());
		mock.assert_async().await;
	}

	#[tokio::test]
	async fn test_rest_put() {
		let mut server = Server::new_async().await;
		let mock = server.mock("PUT", "/some/path").with_status(200).create_async().await;

		let mut client = create_test_client(&server.url()).await;
		let mut params = std::collections::HashMap::new();
		params.insert("key", "value");
		let res = client.rest_put("/some/path".to_string(), params).await;
		assert!(res.is_ok());
		mock.assert_async().await;
	}

	#[tokio::test]
	async fn test_rest_delete() {
		let mut server = Server::new_async().await;
		let mock = server
			.mock("DELETE", "/some/path")
			.with_status(204)
			.create_async()
			.await;

		let mut client = create_test_client(&server.url()).await;
		let res = client.rest_delete("/some/path".to_string()).await;
		assert!(res.is_ok());
		mock.assert_async().await;
	}

	// --- rest_get_fulluri ---

	#[tokio::test]
	async fn test_rest_get_fulluri() {
		let mut server = Server::new_async().await;
		let mock = server
			.mock("GET", "/services/apexrest/MyEndpoint")
			.match_query(mockito::Matcher::UrlEncoded("param".into(), "value".into()))
			.with_status(200)
			.with_body("ok")
			.create_async()
			.await;

		let mut client = create_test_client(&server.url()).await;
		let res = client.rest_get_fulluri("MyEndpoint?param=value").await;
		assert!(res.is_ok());
		mock.assert_async().await;
	}

	// --- ensure_refresh with expired token ---

	#[tokio::test]
	async fn test_ensure_refresh_expired_token_triggers_refresh() {
		let mut server = Server::new_async().await;
		let mock = server
			.mock("POST", "/services/oauth2/token")
			.with_status(200)
			.with_header("content-type", "application/json")
			.with_body(
				json!({
					"access_token": "refreshed_token",
					"issued_at": "9999999999000",
					"id": "https://login.salesforce.com/id/00Dxx/005xx",
					"instance_url": server.url(),
					"signature": "sig",
					"token_type": "Bearer",
				})
				.to_string(),
			)
			.create_async()
			.await;

		let mut credentials = Credentials::new();
		credentials
			.set_login_endpoint(&server.url())
			.set_client_id("cid")
			.set_client_secret("csecret")
			.set_refresh_token("rtoken")
			.set_access_token(Some(AccessToken::new(
				"old_token".to_string(),
				"1000000000000".to_string(), // ~2001, well past 2 hours
				"Bearer".to_string(),
			)));
		let mut client = Client::new(credentials).await.unwrap();
		// Set a token with issued_at far in the past (expired > 2 hours ago)

		let result = client.ensure_refresh().await;
		assert!(result.is_ok());
		assert_eq!(client.credentials.access_token_value(), Some("refreshed_token"));
		mock.assert_async().await;
	}
}
