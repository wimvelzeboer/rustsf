use anyhow::{anyhow, Context, Result};
use std::sync::{Arc, RwLock};
use crate::client::login_with_credential;
use crate::client::responses::access_token;
use crate::client::responses::access_token::AccessToken;
use crate::client::responses::token_response::TokenResponse;
use crate::Credentials;
use crate::credentials::CredentialsType;

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
	pub async fn new(credentials: Credentials) -> Result<Self> {
		let http_client = reqwest::Client::new();

		let (access_token, instance_url, org_id, user_id) =
			match credentials.credentials_type() {
				CredentialsType::AuthUrl => login_with_sfdx_auth_url(&credentials)
					.await
					.context("Failed to login with SFDX Authentication URL")?,
				CredentialsType::ClientCredentials => login_with_credential(&credentials)
					.await
					.context("Failed to login with client credentials")?,
		};

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

fn get_organisation_and_user_id(url: &str) -> Result<(String, String)> {
	let path = url.split_once("/id/")?.1;
	let mut parts = path.split('/');

	let organisation_id = parts.next()?.trim();
	let user_id = parts.next()?.trim();

	if organisation_id.is_empty() || user_id.is_empty() {
		return Err(anyhow!("Unable to retrieve organisation or user id"));
	}

	Ok((organisation_id.to_string(), user_id.to_string()))
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


async fn login_with_sfdx_auth_url(credentials: &Credentials) -> Result<(AccessToken, String, String, String)> {
	let response = get_refresh_token_response(&credentials).await?;

	let (org_id, user_id) = get_organisation_and_user_id(response.id.as_str())?;
	if credentials.user_id.is_none() || credentials.organisation_id.is_none() {
		match get_organisation_and_user_id(response.id.as_str()) {
			Some((org_id, user_id)) => {
				credentials.set_organisation_id(&org_id);
				credentials.set_user_id(&user_id);
			}
			None => {
				log::warn!("Could not extract organisation and user ID from token response.");
			}
		}
	}

	let instance_url = response.instance_url.to_string();
	let access_token = AccessToken::from_token_response(&response);
	Ok((access_token, instance_url, org_id, user_id))
}
