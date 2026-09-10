pub use crate::authentication::credentials::Credentials;
use crate::client::responses::access_token::AccessToken;
use crate::authentication::credentials::CredentialsType;
use anyhow::{anyhow, Context, Result};
use std::sync::{Arc, RwLock};
use crate::client::responses::token_response::TokenResponse;

pub(crate) mod access_token;
pub(crate) mod credentials;
pub(crate) mod token_response;

pub const CONNECT_TIMEOUT: u64 = 15;
pub const REQUEST_TIMEOUT: u64 = 30;


pub(crate) struct Client {
	credentials: Credentials,
	access_token: Arc<RwLock<AccessToken>>,
	http_client: reqwest::Client,
	instance_url: String,
	org_id: String,
	user_id: String,
	version: String,
}

impl Client {
	pub async fn connect(credentials: Credentials) -> Result<Self> {
		let http_client = get_http_client()?;

		let token_response = match credentials.credentials_type() {
			CredentialsType::AuthUrl => get_refresh_token_response(&http_client, &credentials)
				.await
				.context("Failed to login with SFDX Authentication URL")?,
			CredentialsType::ClientCredentials => connect_with_credential(&http_client, &credentials)
				.await
				.context("Failed to login with client credentials")?,
		};

		let (org_id, user_id) = get_organisation_and_user_id(token_response.id.as_str())?;
		let instance_url = token_response.instance_url.to_string();
		let access_token = AccessToken::from_token_response(&token_response);

		Ok(Self {
			credentials,
			access_token: Arc::new(RwLock::new(access_token)),
			http_client,
			instance_url,
			org_id,
			user_id,
			version: "v67.0".to_string(),
		})
	}
}

pub async fn connect_with_credential(
	client: &reqwest::Client,
	credentials: &Credentials
) -> Result<TokenResponse> {

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

	let organisation_id = parts.next()?.trim();
	let user_id = parts.next()?.trim();

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

	Ok(response.json().await.context("Failed to parse refresh token response")?)
}
