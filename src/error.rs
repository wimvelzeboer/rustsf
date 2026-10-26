use rustsf_auth::SalesforceAuthError;
use std::fmt;
use reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
use crate::ErrorResponse;

#[derive(Debug)]
pub enum Error {
	Authentication(SalesforceAuthError),
	Http(reqwest::Error),
	Salesforce(Vec<ErrorResponse>),
	Other(String),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::Authentication(e) => write!(f, "Authentication error: {}", e),
			Error::Http(e) => write!(f, "Http error: {}", e),
			Error::Salesforce(e) => write!(f, "Salesforce error: {:?}", e),
			Error::Other(e) => write!(f, "Error: {}", e),
		}
	}
}

impl std::error::Error for Error {}


impl From<reqwest::Error> for Error {
	fn from(err: reqwest::Error) -> Self {
		Error::Http(err)
	}
}

impl From<InvalidHeaderName> for Error {
	fn from(err: InvalidHeaderName) -> Self {
		Error::Other(format!("Invalid header name: {}", err))
	}
}

impl From<InvalidHeaderValue> for Error {
	fn from(err: InvalidHeaderValue) -> Self {
		Error::Other(format!("Invalid header value: {}", err))
	}
}

impl From<serde_json::Error> for Error {
	fn from(err: serde_json::Error) -> Self {
		Error::Other(format!("Json error: {}", err))
	}
}

impl From<chrono::ParseError> for Error {
	fn from(err: chrono::ParseError) -> Self {
		Error::Other(format!("Date/time parse error: {}", err))
	}
}

impl From<SalesforceAuthError> for Error {
	fn from(err: SalesforceAuthError) -> Self {
		Error::Authentication(err)
	}
}

