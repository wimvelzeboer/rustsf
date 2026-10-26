use std::collections::HashMap;
use std::fmt::Debug;
use log::debug;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION};
use reqwest::Response;
use rustsf_auth::{SalesforceAuthSession, SalesforceCredentials};
use serde::Serialize;
use crate::rest_api::DEFAULT_API_VERSION;
use super::error::RestApiError;

pub const CONNECT_TIMEOUT: u64 = 15;
pub const REQUEST_TIMEOUT: u64 = 30;

pub struct Client {
	http_client: reqwest::Client,
	session: SalesforceAuthSession,
}

impl Client {
	pub async fn new(session: SalesforceCredentials) -> Result<Self, RestApiError> {
		let http_client = reqwest::Client::builder()
			.redirect(reqwest::redirect::Policy::none())
			.connect_timeout(std::time::Duration::from_secs(CONNECT_TIMEOUT))
			.timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT))
			.build()?;

		Ok(Self {
			http_client,
			session: session.connect().await?,
		})
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
	///   - If refreshing the client token fails.
	///   - If there is an issue creating the HTTP headers.
	///   - If the HTTP request fails during execution.
	///
	/// # Async Behavior
	/// - This function is asynchronous and should be awaited.
	/// - Before sending the DELETE request, it ensures that the client state is up-to-date by calling `self.ensure_refresh()`.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{Client, SalesforceCredentials};
	/// use std::collections::HashMap;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
	///     let credentials = SalesforceCredentials::sfdx_url("force://PlatformCLI::token@example.com")?;
	///     let client = Client::new(credentials).await?;
	///
	///     let response = client.delete("https://example.com/services/data/v62.0/sobjects/Account/001...", &HashMap::new()).await?;
	///     println!("Status: {}", response.status());
	///     Ok(())
	/// }
	/// ```
	pub async fn delete(&self, url: &str, headers: &HashMap<String, String>) -> anyhow::Result<Response, RestApiError> {
		debug!("DELETE: {} headers: {:?}", url, headers);

		let response = self
			.http_client
			.delete(url)
			.headers(self.create_header(headers).await?)
			.send()
			.await?;

		debug!("DELETE response: {:?}", response);
		Ok(response)
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
	/// use rustsf::{Client, SalesforceCredentials};
	/// use std::collections::HashMap;
	/// use serde_json::json;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
	///     let credentials = SalesforceCredentials::sfdx_url("force://PlatformCLI::token@example.com")?;
	///     let client = Client::new(credentials).await?;
	///
	///     let response = client.get(
	///         "https://example.com/services/data/v62.0/query",
	///         &json!({"q": "SELECT Id FROM Account"}),
	///         &HashMap::new()
	///     ).await?;
	///     println!("Status: {}", response.status());
	///     Ok(())
	/// }
	/// ```
	pub async fn get<T: Debug + Serialize>(
		&self,
		url: &str,
		params: &T, // todo should be made optional, as some calls do not require this.
		headers: &HashMap<String, String>,
	) -> Result<Response, RestApiError> {

		debug!("GET: {} params: {:?} headers: {:?}", url, params, headers);

		let response = self
			.http_client
			.get(url)
			.headers(self.create_header(headers).await?)
			.query(&params)
			.send()
			.await?;

		debug!("GET response: {:?}", response);
		Ok(response)
	}

	pub(crate) fn instance_url(&self) -> &str {
		&self.session.instance_url
	}

	pub fn org_id(&self) -> Option<&str> {
		self.session.org_id.as_deref()
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
	/// - If the `ensure_refresh` internal function fails (e.g., unable to refresh client tokens).
	/// - If there is an error serializing the parameters or constructing the HTTP headers.
	/// - If the HTTP request fails during transmission or response handling.
	///
	/// # Examples
	/// ```rust
	/// use serde::Serialize;
	/// use rustsf::{Client, SalesforceCredentials};
	/// use std::collections::HashMap;
	///
	/// #[derive(Serialize, Debug)]
	/// struct UpdateParams {
	///     field: String,
	///     value: String,
	/// }
	///
	/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
	///     let credentials = SalesforceCredentials::sfdx_url("force://PlatformCLI::token@example.com")?;
	///     let client = Client::new(credentials).await?;
	///
	///     let params = UpdateParams {
	///         field: "Name".to_string(),
	///         value: "New Name".to_string(),
	///     };
	///
	///     let response = client
	///         .patch("https://example.com/services/data/v62.0/sobjects/Account/001...", params, &HashMap::new())
	///         .await?;
	///
	///     println!("Response: {:?}", response);
	///     Ok(())
	/// }
	/// ```
	pub async fn patch<T: Debug + Serialize>(
		&self,
		url: &str,
		params: T,
		headers: &HashMap<String, String>,
	) -> Result<Response, RestApiError> {

		debug!("PATCH: {} params: {:?} headers: {:?}", url, params, headers);

		Ok(self
			.http_client
			.patch(url)
			.headers(self.create_header(headers).await?)
			.json(&params)
			.send()
			.await?)
	}

	pub async fn post<T: Debug + Serialize>(
		&self,
		url: &str,
		data: &T,
		headers: &HashMap<String, String>,
	) -> Result<reqwest::Response, RestApiError> {

		debug!("POST: {} params: {:?} headers: {:?}", url, data, headers);

		Ok(self
			.http_client
			.post(url)
			.headers(self.create_header(headers).await?)
			.json(&data)
			.send()
			.await?)
	}

	pub fn services_data_path(&self) -> String {
		format!("{}/services/data", self.session.instance_url)
	}

	pub(crate) fn services_data_version_path(&self) -> String {
		format!("{}/services/data/v{}", self.session.instance_url, DEFAULT_API_VERSION)
	}

	pub fn user_id(&self) -> Option<&str> {
		self.session.user_id.as_deref()
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
	async fn create_header(&self, additional_headers: &HashMap<String, String>) -> Result<HeaderMap, RestApiError> {
		let mut headers = HeaderMap::new();

		let auth_value = format!("Bearer {}", self.session.access_token().await?);
		headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_value)?);

		//Default header
		headers.insert("Accept", HeaderValue::from_static("application/json"));

		for (key, value) in additional_headers {
			let header_name: HeaderName = key.parse()?;

			let header_value = HeaderValue::from_str(&value)?;

			//delete duplicates
			headers.remove(&header_name);

			headers.insert(header_name, header_value);
		}

		Ok(headers)
	}
}

