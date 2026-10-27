use rustsf_auth::SalesforceAuthError;
use std::fmt;
use reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
use zip::result::ZipError;
use crate::ErrorResponse;

#[cfg(feature = "metadata-api")]
mod xml_error;

#[cfg(feature = "metadata-api")]
pub use xml_error::XmlError;

#[derive(Debug)]
pub enum Error {
	Authentication(SalesforceAuthError),
	Json(serde_json::Error),
	Http(reqwest::Error),
	Salesforce(Vec<ErrorResponse>),
	Other(String),

	#[cfg(feature = "metadata-api")]
	CompressionError(String),

	#[cfg(feature = "metadata-api")]
	Xml(XmlError),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::Authentication(e) => write!(f, "Authentication error: {}", e),
			Error::Http(e) => write!(f, "Http error: {}", e),
			Error::Json(e) => write!(f, "Json error: {}", e),
			Error::Salesforce(e) => write!(f, "Salesforce error: {:?}", e),

			Error::Other(e) => write!(f, "Error: {}", e),

			#[cfg(feature = "metadata-api")]
			Error::Xml(e) => write!(f, "XML Error: {}", e),

			#[cfg(feature = "metadata-api")]
			Error::CompressionError(e) => write!(f, "Zip Error: {}", e)
		}
	}
}

impl std::error::Error for Error {}


impl From<chrono::ParseError> for Error {
	fn from(err: chrono::ParseError) -> Self {
		Error::Other(format!("Date/time parse error: {}", err))
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

impl From<reqwest::Error> for Error {
	fn from(err: reqwest::Error) -> Self {
		Error::Http(err)
	}
}

impl From<serde_json::Error> for Error {
	fn from(err: serde_json::Error) -> Self {
		Error::Other(format!("Json error: {}", err))
	}
}


#[cfg(feature = "metadata-api")]
impl From<XmlError> for Error {
	fn from(err: XmlError) -> Self {
		Error::Xml(err)
	}
}

impl From<SalesforceAuthError> for Error {
	fn from(err: SalesforceAuthError) -> Self {
		Error::Authentication(err)
	}
}

impl From<ZipError> for Error {
	fn from(err: ZipError) -> Self {
		Error::CompressionError(err.to_string())
	}
}

