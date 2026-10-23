use rustsf_auth::SalesforceAuthError;
use std::fmt;
use reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
use crate::ErrorResponse;

#[derive(Debug)]
pub enum RestApiError {
	Authentication(SalesforceAuthError),
	Http(reqwest::Error),
	Salesforce(Vec<ErrorResponse>),
	Other(String),
}

impl fmt::Display for RestApiError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			RestApiError::Authentication(e) => write!(f, "Authentication error: {}", e),
			RestApiError::Http(e) => write!(f, "Http error: {}", e),
			RestApiError::Salesforce(e) => write!(f, "Salesforce error: {:?}", e),
			RestApiError::Other(e) => write!(f, "Error: {}", e),
		}
	}
}

impl std::error::Error for RestApiError {
	/*fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			RestApiError::Authentication(err) => Some(err),
			RestApiError::Http(err) => Some(err),
			RestApiError::Salesforce(err) => Some(err),
			RestApiError::Other(err) => Some(err),
		}
	}*/
}

impl From<reqwest::Error> for RestApiError {
	fn from(err: reqwest::Error) -> Self {
		RestApiError::Http(err)
	}
}

impl From<InvalidHeaderName> for RestApiError {
	fn from(err: InvalidHeaderName) -> Self {
		RestApiError::Other(format!("Invalid header name: {}", err))
	}
}

impl From<InvalidHeaderValue> for RestApiError {
	fn from(err: InvalidHeaderValue) -> Self {
		RestApiError::Other(format!("Invalid header value: {}", err))
	}
}

impl From<serde_json::Error> for RestApiError {
	fn from(err: serde_json::Error) -> Self {
		RestApiError::Other(format!("Json error: {}", err))
	}
}

impl From<SalesforceAuthError> for RestApiError {
	fn from(err: SalesforceAuthError) -> Self {
		RestApiError::Authentication(err)
	}
}

