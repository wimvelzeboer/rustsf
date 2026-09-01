//! # Credential builders
//!

use crate::client::responses::access_token::AccessToken;
use serde::{Deserialize, Serialize};

pub mod auth_url;
pub mod client_credentials;

pub mod credential_file;

/// Salesforce OAuth2 credentials
#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
	pub(crate) access_token: Option<AccessToken>,
	client_id: Option<String>,
	client_secret: Option<String>,
	instance_url: Option<String>,
	login_endpoint: String,
	pub organisation_id: Option<String>,
	refresh_token: Option<String>,
	pub user_id: Option<String>,
	username: Option<String>,
	password: Option<String>,
}

impl Credentials {
	pub fn new() -> Self {
		Self {
			access_token: None,
			client_id: None,
			client_secret: None,
			instance_url: Some("https://na1.salesforce.com".to_string()),
			login_endpoint: "https://login.salesforce.com".to_string(),
			organisation_id: None,
			refresh_token: None,
			user_id: None,
			username: None,
			password: None,
		}
	}

	/// Retrieves a reference to the `AccessToken` if it exists.
	///
	/// # Returns
	///
	/// - `Some(&AccessToken)` - A reference to the `AccessToken` if it is available.
	/// - `None` - If the `access_token` is not set.
	///
	/// This method provides a way to access the `AccessToken` field of the
	/// struct without taking ownership of it. It is useful when you want
	/// to inspect or work with the token without modifying the struct.
	///
	pub fn access_token(&self) -> Option<&AccessToken> {
		self.access_token.as_ref()
	}

	/// Retrieves the value of the access token, if it exists.
	///
	/// This method accesses the `access_token` field of the struct and extracts its
	/// `value` as a string slice. If the `access_token` is `None`, this method will
	/// return `None`.
	///
	/// # Returns
	/// * `Option<&str>` - `Some(&str)` containing the value of the access token if it exists,
	///   or `None` if the `access_token` is not present.
	///
	pub fn access_token_value(&self) -> Option<&str> {
		self.access_token.as_ref().map(|t| t.value.as_str())
	}

	/// Retrieves the client ID associated with the current object, if it exists.
	///
	/// # Returns
	/// - `Option<&str>`:
	///    - `Some(&str)` containing the reference to the client ID if it is present.
	///    - `None` if the client ID is not set.
	///
	///
	/// This method allows for safely accessing the client ID without taking ownership of the value.
	pub fn client_id(&self) -> Option<&str> {
		self.client_id.as_deref()
	}

	/// Retrieves the client secret as an optional reference to a string slice.
	///
	/// # Returns
	/// - `Some(&str)` if the `client_secret` field is set in the struct.
	/// - `None` if the `client_secret` field is `None`.
	///
	/// This method allows safe access to the client secret without consuming the value.
	///
	pub fn client_secret(&self) -> Option<&str> {
		self.client_secret.as_deref()
	}

	pub fn get_flow_type(&self) -> Option<CredentialsType> {
		if self.username.is_some() && self.password.is_some() {
			Some(CredentialsType::Password)
		} else if self.refresh_token.is_some() {
			Some(CredentialsType::AuthUrl)
		} else if self.client_id.is_some() && self.client_secret.is_some() {
			Some(CredentialsType::ClientCredentials)
		} else {
			None
		}
	}

	pub fn get_instance_url(&self) -> Option<&str> {
		self.instance_url.as_deref()
	}

	pub fn has_access_token(&self) -> bool {
		self.access_token.is_some()
	}

	pub fn is_access_token_expired(&self) -> bool {
		match &self.access_token {
			Some(access_token) => access_token.is_expired(),
			None => true,
		}
	}

	/// Returns the login endpoint URL as a string slice.
	///
	///
	/// # Returns
	/// * `&str` - A reference to the login endpoint URL.
	pub fn login_endpoint(&self) -> &str {
		&self.login_endpoint
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
	pub fn instance_url(&self) -> Option<&str> {
		self.instance_url.as_deref()
	}

	pub fn password(&mut self) -> Option<&str> {
		self.password.as_deref()
	}

	/// Returns an optional reference to the refresh token.
	///
	/// This method provides a way to retrieve the refresh token, returning it as an
	/// `Option<&str>`. If a refresh token exists within the structure, it is returned
	/// as a `Some(&str)`; otherwise, `None` is returned.
	///
	/// # Returns
	/// - `Some(&str)` if the refresh token is present.
	/// - `None` if the refresh token is absent.
	///
	pub fn refresh_token(&self) -> Option<&str> {
		self.refresh_token.as_deref()
	}

	/// Sets the access token for the current instance.
	///
	/// This function updates the access token along with its type and issuance time.
	/// It allows method chaining by returning a mutable reference to the instance.
	///
	/// # Arguments
	///
	/// * `access_token` - A `String` containing the access token value.
	/// * `issued_at` - A `String` representing the time at which the token was issued.
	/// * `token_type` - A `String` denoting the type of the token, e.g., "Bearer".
	///
	/// # Returns
	///
	/// A mutable reference to the current instance (`&mut Self`), enabling method chaining.
	///
	///
	/// This sets the access token, token type, and issuance time for the client.
	pub fn set_access_token(&mut self, access_token: Option<AccessToken>) -> &mut Self {
		self.access_token = access_token;
		self
	}

	pub fn set_client_id(&mut self, id: &str) -> &mut Self {
		self.client_id = Some(id.to_string());
		self
	}

	pub fn set_client_secret(&mut self, secret: &str) -> &mut Self {
		self.client_secret = Some(secret.to_string());
		self
	}

	pub fn set_instance_url(&mut self, url: &str) -> &mut Self {
		self.instance_url = Some(url.to_string());
		self
	}

	pub fn set_login_endpoint(&mut self, endpoint: &str) -> &mut Self {
		self.login_endpoint = endpoint.to_string();
		self
	}

	pub fn set_organisation_id(&mut self, id: &str) -> &mut Self {
		self.organisation_id = Some(id.to_string());
		self
	}

	pub fn set_refresh_token(&mut self, token: &str) -> &mut Self {
		self.refresh_token = Some(token.to_string());
		self
	}

	pub fn set_user_id(&mut self, id: &str) -> &mut Self {
		self.user_id = Some(id.to_string());
		self
	}

	pub fn username(&mut self) -> Option<&str> {
		self.username.as_deref()
	}
}

#[derive(Debug)]
pub enum CredentialsType {
	AuthUrl,
	ClientCredentials,
	Password,
	None,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new_defaults() {
		let credentials = Credentials::new();
		assert_eq!(credentials.login_endpoint(), "https://login.salesforce.com");
		assert!(credentials.client_id().is_none());
		assert!(credentials.client_secret().is_none());
		assert!(credentials.access_token().is_none());
		assert!(credentials.instance_url().is_some());
		assert!(credentials.refresh_token().is_none());
	}

	#[test]
	fn test_set_instance_url() {
		let mut credentials = Credentials::new();
		credentials.set_instance_url("https://example.com");
		assert_eq!(Some("https://example.com"), credentials.instance_url());
	}

	#[test]
	fn test_set_refresh_token() {
		let mut credentials = Credentials::new();
		credentials.set_refresh_token("my_refresh_token");
		assert_eq!(Some("my_refresh_token"), credentials.refresh_token());
	}

	#[test]
	fn test_set_client_id() {
		let mut credentials = Credentials::new();
		credentials.set_client_id("my_client_id");
		assert_eq!(Some("my_client_id"), credentials.client_id());
	}

	#[tokio::test]
	async fn test_set_client_secret() {
		let mut credentials = Credentials::new();
		credentials.set_client_secret("my_secret");
		assert_eq!(Some("my_secret"), credentials.client_secret());
	}

	#[tokio::test]
	async fn test_setter_chaining() {
		let mut credentials = Credentials::new();
		credentials
			.set_login_endpoint("https://test.salesforce.com")
			.set_instance_url("https://inst.salesforce.com")
			.set_client_id("cid")
			.set_client_secret("csecret")
			.set_refresh_token("rtoken");

		assert_eq!(credentials.login_endpoint(), "https://test.salesforce.com");
		assert_eq!(credentials.instance_url(), Some("https://inst.salesforce.com"));
		assert_eq!(credentials.client_id(), Some("cid"));
		assert_eq!(credentials.client_secret(), Some("csecret"));
		assert_eq!(credentials.refresh_token(), Some("rtoken"));
	}
}
