use crate::errors::Error;
use log::debug;
use reqwest::Response;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderName, HeaderValue};
use rustsf_auth::{SalesforceAuthSession, SalesforceCredentials};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;

#[cfg(feature = "metadata-api")]
use reqwest::multipart::Form;

pub const DEFAULT_API_VERSION: &str = "68.0";

pub const CONNECT_TIMEOUT: u64 = 15;
pub const REQUEST_TIMEOUT: u64 = 30;

pub struct Client {
	http_client: reqwest::Client,
	session: SalesforceAuthSession,
}

impl Client {
	pub async fn new(session: SalesforceCredentials) -> Result<Self, Error> {
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

	#[cfg(feature = "metadata-api")]
	pub async fn access_token(&self) -> Result<String, Error> {
		Ok(self.session.access_token().await?)
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
	pub async fn delete(&self, url: &str, headers: &HashMap<String, String>) -> Result<Response, Error> {
		debug!("DELETE: {} headers: {:?}", url, headers);

		let mut headers = self.create_header(headers).await?;
		headers.insert("Content-Type", HeaderValue::from_static("application/json"));

		let response = self.http_client.delete(url).headers(headers).send().await?;

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
	pub async fn get_json<T: Debug + Serialize, S: AsRef<str> + reqwest::IntoUrl>(
		&self,
		url: S,
		params: &T, // todo should be made optional, as some calls do not require this.
		headers: &HashMap<String, String>,
	) -> Result<Response, Error> {
		debug!("GET: {} params: {:?} headers: {:?}", url.as_ref(), params, headers);

		let mut headers = self.create_header(headers).await?;
		headers.insert("Content-Type", HeaderValue::from_static("application/json"));

		let response = self.http_client.get(url).headers(headers).query(&params).send().await?;

		debug!("GET response: {:?}", response);
		Ok(response)
	}

	#[cfg(feature = "bulk-api-v2")]
	pub async fn get_raw(
		&mut self,
		url: &str,
		additional_headers: &HashMap<String, String>,
	) -> Result<Response, Error> {
		debug!("GET: {} headers: {:?}", url, additional_headers);

		let res = self
			.http_client
			.get(url)
			.headers(self.create_header(additional_headers).await?)
			.send()
			.await?;
		Ok(res)
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
	pub async fn patch<T: Debug + Serialize>(
		&self,
		url: &str,
		params: T,
		headers: &HashMap<String, String>,
	) -> Result<Response, Error> {
		debug!("PATCH: {} params: {:?} headers: {:?}", url, params, headers);

		let mut headers = self.create_header(headers).await?;
		headers.insert("Content-Type", HeaderValue::from_static("application/json"));

		Ok(self
			.http_client
			.patch(url)
			.headers(headers)
			.json(&params)
			.send()
			.await?)
	}

	pub async fn post_json<T: Debug + Serialize>(
		&self,
		url: &str,
		data: &T,
		headers: &HashMap<String, String>,
	) -> Result<Response, Error> {
		debug!("POST: {} params: {:?} headers: {:?}", url, data, headers);

		let mut headers = self.create_header(headers).await?;
		headers.insert("Content-Type", HeaderValue::from_static("application/json"));

		Ok(self.http_client.post(url).headers(headers).json(&data).send().await?)
	}

	pub async fn post(&self, url: &str, data: Vec<u8>, headers: &HashMap<String, String>) -> Result<Response, Error> {
		debug!("POST: {} headers: {:?}", url, headers);

		Ok(self
			.http_client
			.post(url)
			.headers(self.create_header(headers).await?)
			.body(data)
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
	pub async fn post_multipart<S>(
		&self,
		url: S,
		headers: &HashMap<String, String>,
		form: Form,
	) -> Result<Response, Error>
	where
		S: AsRef<str> + reqwest::IntoUrl,
	{
		let mut headers = self.create_header(headers).await?;
		headers.insert("Content-Type", HeaderValue::from_static("application/json"));

		let request = self.http_client.post(url).multipart(form).headers(headers).build()?;

		debug!("Request URL: {}", request.url());
		debug!("Request Method: {}", request.method());

		// Log body size (reqwest hides the exact body bytes if it's a stream/json,
		// but you can check if a body exists)
		if let Some(body) = request.body() {
			debug!("Body: {:?}", body.as_bytes());
		}

		let response = self.http_client.execute(request).await?;

		Ok(response)
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
	pub async fn put<T: Into<reqwest::Body>>(&mut self, url: String, buffer: T) -> Result<Response, Error> {
		debug!("PUT: {}", url);

		let headers: HashMap<String, String> = HashMap::new();
		let mut headers = self.create_header(&headers).await?;
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

	pub async fn post_soap(&self, action: &str, body: String) -> Result<Response, Error> {
		let url = self.soap_version_path();
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

	pub fn services_data_path(&self) -> String {
		format!("{}/services/data", self.instance_url())
	}

	pub(crate) fn services_data_version_path(&self) -> String {
		format!("{}/services/data/v{}", self.instance_url(), DEFAULT_API_VERSION)
	}

	pub fn soap_version_path(&self) -> String {
		let instance_url = self.instance_url();
		format!("{}/services/Soap/m/{}", instance_url, DEFAULT_API_VERSION)
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
	async fn create_header(&self, additional_headers: &HashMap<String, String>) -> Result<HeaderMap, Error> {
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
