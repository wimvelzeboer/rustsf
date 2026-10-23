use log::trace;
use serde::de::DeserializeOwned;
use crate::ErrorResponse;
use crate::rest_api::error::RestApiError;

pub struct RestResponse {
	response: reqwest::Response,
}

impl RestResponse {

	pub fn new(response: reqwest::Response) -> Self {
		Self { response }
	}

	pub async fn errors(self) -> Result<Vec<ErrorResponse>, reqwest::Error> {
		self.response.json().await
	}

	pub fn etag(&self) -> Option<String> {
		self.response.headers().get("etag").and_then(|etag| Some(etag.to_str().unwrap().to_string()))
	}

	pub fn headers(&self) -> &reqwest::header::HeaderMap {
		self.response.headers()
	}

	pub fn is_success(&self) -> bool {
		self.response.status().is_success()
	}

	pub async fn json<T: serde::de::DeserializeOwned>(self) -> Result<T, reqwest::Error> {
		self.response.json().await
	}

	pub fn limit_info(&self) -> Option<&str> {
		self.response.headers().get("sforce-limit-info").and_then(|sforce_limit_info| sforce_limit_info.to_str().ok())
	}

	pub fn response(self) -> reqwest::Response {
		self.response
	}

	pub async fn rest_response<T: DeserializeOwned>(self) -> Result<T, RestApiError> {
		if self.response.status().is_success() {
			let json_str = self.response.text().await?;
			trace!("RESPONSE BODY: {}", json_str);
			Ok(serde_json::from_str(&json_str)?)
		} else {
			let errors: Vec<ErrorResponse> = self.response.json().await?;
			Err(RestApiError::Salesforce(errors))
		}
	}

	pub fn status(&self) -> reqwest::StatusCode {
		self.response.status()
	}

	pub async fn text(self) -> Result<String, reqwest::Error> {
		self.response.text().await
	}
}