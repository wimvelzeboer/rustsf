use self::access_token::AccessToken;
pub(crate) use self::credentials::{Credentials, CredentialsType};
use self::token_response::TokenResponse;
use crate::ErrorResponse;
use anyhow::{Context, Result, anyhow};
use log::debug;
use reqwest::Response;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

pub(crate) mod access_token;
pub(crate) mod credentials;
pub(crate) mod token_response;

pub const CONNECT_TIMEOUT: u64 = 15;
pub const REQUEST_TIMEOUT: u64 = 30;

pub struct Client {
	credentials: Credentials,
	access_token: Arc<RwLock<AccessToken>>,
	http_client: reqwest::Client,
	instance_url: String,
	org_id: String,
	user_id: String,
	api_version: String,
}

impl Client {
	pub fn api_version(&self) -> &str {
		&self.api_version
	}

	pub async fn connect(credentials: Credentials) -> Result<Self> {
		let http_client = get_http_client()?;

		// Mocking
		if credentials.login_endpoint().ends_with("example.com") {
			return Ok(Self {
				credentials,
				access_token: Arc::new(RwLock::new(AccessToken::new("token".to_string(), "0".to_string(), "barrier".to_string()))),
				http_client,
				instance_url: "https://www.example.com".to_string(),
				org_id: "org_id".to_string(),
				user_id: "user_id".to_string(),
				api_version: "v67.0".to_string(),
			})
		}

		let token_response = match credentials.credentials_type() {
			CredentialsType::AuthUrl => get_refresh_token_response(&http_client, &credentials)
				.await
				.context("Authentication using SFDX Authentication URL failed")?,
			CredentialsType::ClientCredentials => connect_with_credential(&http_client, &credentials)
				.await
				.context("Authentication using client credentials failed")?,

			#[cfg(test)]
			CredentialsType::Test => {
				let json_str = json!({
					"access_token": "token123",
					"issued_at": "1234567890",
					"id": "https://login.salesforce.com/id/00Dxx/005xx",
					"instance_url": "https://na1.salesforce.com",
					"signature": "sig",
					"token_type": "Bearer"
				})
				.to_string();

				serde_json::from_str::<TokenResponse>(&json_str)?
			}
		};

		let (org_id, user_id) = get_organisation_and_user_id(token_response.id.as_str())?;
		let instance_url = token_response.instance_url.to_string();
		let access_token = AccessToken::from_token_response(token_response);

		Ok(Self {
			credentials,
			access_token: Arc::new(RwLock::new(access_token)),
			http_client,
			instance_url,
			org_id,
			user_id,
			api_version: "v67.0".to_string(),
		})
	}

	pub async fn get<T: Debug + Serialize>(
		&self,
		url: &str,
		params: &T,
		headers: &HashMap<String, String>,
	) -> Result<Response> {
		debug!("GET: {} params: {:?} headers: {:?}", url, params, headers);

		let response = self
			.http_client
			.get(url)
			.headers(self.create_header(headers)?)
			.query(&params)
			.send()
			.await?;

		debug!("GET response: {:?}", response);
		Ok(response)
	}

	pub fn instance_url(&self) -> &str {
		&self.instance_url
	}

	pub async fn post<T: Debug + Serialize>(
		&self,
		url: &str,
		data: &T,
		headers: &HashMap<String, String>,
	) -> Result<reqwest::Response> {
		self.ensure_connection().await?;

		debug!("POST: {} params: {:?} headers: {:?}", url, data, headers);

		Ok(self
			.http_client
			.post(url)
			.headers(self.create_header(headers)?)
			.json(&data)
			.send()
			.await?)
	}

	pub fn services_data_version_path(&self) -> String {
		format!("{}/services/data/{}", self.instance_url, self.api_version)
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
	fn create_header(&self, additional_headers: &HashMap<String, String>) -> Result<HeaderMap> {
		let mut headers = HeaderMap::new();

		let guard = self.access_token.read().unwrap_or_else(|e| e.into_inner());
		let access_token = guard;

		let auth_value = format!("Bearer {}", access_token.value);
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
	async fn ensure_connection(&self) -> Result<()> {
		match self.access_token.read() {
			Ok(guard) if guard.is_expired() == false => return Ok(()),
			_ => {}
		}

		log::info!("Access Token Expired, Refreshing.");

		self.refresh_access_token()
			.await
			.context("Failed to refresh access token")
	}

	async fn refresh_access_token(&self) -> Result<()> {
		let response = get_refresh_token_response(&self.http_client, &self.credentials).await?;

		let mut guard = self.access_token.write().unwrap_or_else(|e| e.into_inner());

		*guard = AccessToken::from_token_response(response);

		Ok(())
	}
}

pub async fn connect_with_credential(client: &reqwest::Client, credentials: &Credentials) -> Result<TokenResponse> {
	let token_url = format!("{}/services/oauth2/token", &credentials.login_endpoint());
	let client_id = &credentials
		.client_id()
		.context("Client ID is not configured")?
		.to_string();
	let client_secret = &credentials
		.client_secret()
		.context("Client Secret is not configured")?
		.to_string();
	let username = &credentials.username().unwrap_or("").to_string();
	let password = &credentials.password().unwrap_or("").to_string();

	let params = [
		("grant_type", "password"),
		("client_id", client_id.as_str()),
		("client_secret", client_secret.as_str()),
		("username", username.as_str()),
		("password", password.as_str()),
	];

	let res = client
		.post(token_url.as_str())
		.form(&params)
		.send()
		.await
		.context("Failed to get token")?;

	if !res.status().is_success() {
		let error_response = res
			.text()
			.await
			.context("Failed to parse refresh token error response")?;
		return Err(anyhow!("Failed to get refresh token: {}", error_response));
	}

	Ok(res.json().await.context("Failed to parse token response")?)
}

pub fn get_http_client() -> Result<reqwest::Client> {
	Ok(reqwest::Client::builder()
		.redirect(reqwest::redirect::Policy::none())
		.connect_timeout(std::time::Duration::from_secs(CONNECT_TIMEOUT))
		.timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT))
		.build()
		.context("Failed to build HTTP client")?)
}

fn get_organisation_and_user_id(url: &str) -> Result<(String, String)> {
	let path = url.split_once("/id/").with_context(|| "Unable to split URL")?.1;
	let mut parts = path.split('/');

	let organisation_id = parts.next().unwrap_or("").trim();
	let user_id = parts.next().unwrap_or("").trim();

	if organisation_id.is_empty() || user_id.is_empty() {
		return Err(anyhow!("Unable to retrieve organisation or user id"));
	}

	Ok((organisation_id.to_string(), user_id.to_string()))
}

async fn get_refresh_token_response(client: &reqwest::Client, credentials: &Credentials) -> Result<TokenResponse> {
	let url = format!("{}/services/oauth2/token", credentials.login_endpoint());
	let params = [
		("grant_type", "refresh_token"),
		("client_id", credentials.client_id().context("Missing client id")?),
		(
			"refresh_token",
			credentials.refresh_token().context("Missing refresh token")?,
		),
	];
	let response = client
		.post(url)
		.form(&params)
		.send()
		.await
		.context("Failed to refresh access token")?;

	if !response.status().is_success() {
		let error_response = response
			.text()
			.await
			.context("Failed to parse refresh token error response")?;
		return Err(anyhow!("Failed to get refresh token: {}", error_response));
	}

	Ok(response
		.json()
		.await
		.context("Failed to parse refresh token response")?)
}

pub async fn handle_json_response<T: DeserializeOwned>(response: Response) -> Result<T> {
	if response.status().is_success() {
		Ok(response.json().await.context("Failed to parse response")?)
	} else {
		let errors: Vec<ErrorResponse> = response.json().await.context("Failed to parse error response")?;
		Err(anyhow!("Response error {:?}", errors))
	}
}

pub async fn handle_empty_response(response: Response) -> Result<()> {
	if response.status().is_success() {
		Ok(())
	} else {
		let errors: Vec<ErrorResponse> = response.json().await?;
		Err(anyhow!("Response error {:?}", errors))
	}
}
