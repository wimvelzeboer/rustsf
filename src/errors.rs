//! Error types for the rustsf crate.
//!
//! This module defines the `Error` enum which represents all possible errors
//! that can occur when interacting with the Salesforce API, including:
//! - Authentication errors (login, token)
//! - HTTP communication errors
//! - Response parsing errors
//! - Configuration errors
//! - API-specific error responses



/// An enumeration representing various errors that can occur in the application.
///
/// This `Error` enum is marked as `#[non_exhaustive]`, which means new variants
/// may be added in future releases. When matching against variants of this enum,
/// an extra `_` wildcard arm must be added to account for any future additions.
///
/// # Variants
///
/// - `NotLoggedIn`
///     - Returned when an operation is attempted without being logged in.
///
/// - `ConfigError(String)`
///     - Represents an error related to configuration, carrying a descriptive
///       message with the underlying cause of the error.
///
/// - `TokenError(TokenErrorResponse)`
///     - Indicates an error related to token handling. Contains a `TokenErrorResponse`
///       that provides additional details about the failure.
///
/// - `HttpError(reqwest::Error)`
///     - Represents an HTTP-related error, wrapping the underlying `reqwest::Error`
///       that provides details of the HTTP issue.
///
/// - `HeaderError(InvalidHeaderValue)`
///     - Signifies an error related to HTTP headers. Wraps an `InvalidHeaderValue`
///       to indicate issues with a header value.
///
/// - `DeserializeError(serde_json::Error)`
///     - Occurs when deserialization of data fails, wrapping a `serde_json::Error`
///       with more details about the failure during parsing.
///
/// - `ErrorResponses(Vec<ErrorResponse>)`
///     - Contains a collection of error responses, represented as `Vec<ErrorResponse>`,
///       encountered during an operation.
///
/// - `DescribeError(ErrorResponse)`
///     - Represents an error that occurs during a describe operation. Wraps a single
///       `ErrorResponse` detailing the problem.
///
/// - `LoginError(LoginErrorResponse)`
///     - Returned when a login operation fails, containing a `LoginErrorResponse`
///       for further details on the failure.
///
/// # Notes
///
/// When working with this enum, consider handling all variants appropriately or using
/// the `_` wildcard to guard against future changes due to the `#[non_exhaustive]`
/// attribute.
///
/// # Example
///
/// ```rust
/// use rustsf::Error;
///
/// fn handle_error(err: Error) {
///     match err {
///         Error::NotLoggedIn => println!("User is not logged in!"),
///         Error::ConfigError(message) => println!("Configuration error: {}", message),
///         Error::TokenError(_) => println!("Token error occurred."),
///         Error::HttpError(e) => println!("HTTP error: {}", e),
///         _ => println!("An unknown error occurred."),
///     }
/// }
/// ```
/*
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    NotLoggedIn,
    ConfigError(String),
    TokenError(TokenErrorResponse),
    HttpError(reqwest::Error),
    HeaderError(InvalidHeaderValue),
    DeserializeError(serde_json::Error),

    #[cfg(feature = "rest-api")]
    ErrorResponses(Vec<ErrorResponse>),

    #[cfg(feature = "rest-api")]
    DescribeError(ErrorResponse),
    LoginError(LoginErrorResponse),
    ResponseError(ResponseError),
    RequestError(String),

    #[cfg(feature = "metadata-api")]
    MetadataError(String),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::HttpError(e) => Some(e),
            Error::HeaderError(e) => Some(e),
            Error::DeserializeError(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NotLoggedIn => write!(f, "Not logged in"),
            Error::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            Error::TokenError(resp) => write!(f, "Invalid token {:?}", resp),
            Error::HttpError(e) => write!(f, "HTTP request to Salesforce failed: {}", e),
            Error::HeaderError(e) => write!(f, "Invalid header value: {}", e),
            Error::DeserializeError(e) => write!(f, "Could not deserialize response: {}", e),
            Error::ErrorResponses(resp) => write!(f, "Error response from Salesforce {:?}", resp),
            Error::DescribeError(resp) => write!(f, "Error completing describe {:?}", resp),
            Error::LoginError(resp) => write!(f, "Error logging in {:?}", resp),
            Error::ResponseError(resp) => write!(f, "Error in Salesforce response {:?}", resp),
            Error::RequestError(msg) => write!(f, "Error in Salesforce request {:?}", msg),

            #[cfg(feature = "metadata-api")]
            Error::MetadataError(msg) => write!(f, "Error in Salesforce metadata request {:?}", msg),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::HttpError(e)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(e: InvalidHeaderValue) -> Self {
        Error::HeaderError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::DeserializeError(e)
    }
}

#[cfg(feature = "metadata-api")]
impl From<ZipError> for Error {
    fn from(value: ZipError) -> Self {
        Error::MetadataError(value.to_string())
    }
}

#[cfg(feature = "metadata-api")]
impl From<XmlParseError> for Error {
    fn from(value: XmlParseError) -> Self {
        Self::MetadataError(value.to_string())
    }
}

*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;

    #[test]
    fn test_display_not_logged_in() {
        let err = Error::NotLoggedIn;
        assert_eq!(format!("{}", err), "Not logged in");
    }

    #[test]
    fn test_display_config_error() {
        let err = Error::ConfigError("client_id is required".to_string());
        assert_eq!(
            format!("{}", err),
            "Configuration error: client_id is required"
        );
    }

    #[test]
    fn test_display_token_error() {
        let token_err: TokenErrorResponse = serde_json::from_str(
            r#"{"error":"invalid_grant","error_description":"bad credentials"}"#,
        )
            .unwrap();
        let err = Error::TokenError(token_err);
        let display = format!("{}", err);
        assert!(display.contains("Invalid token"));
        assert!(display.contains("invalid_grant"));
    }

    #[test]
    fn test_display_deserialize_error() {
        let json_err = serde_json::from_str::<String>("invalid json").unwrap_err();
        let err = Error::DeserializeError(json_err);
        let display = format!("{}", err);
        assert!(display.starts_with("Could not deserialize response: "));
    }

    #[test]
    fn test_display_header_error() {
        let header_err = reqwest::header::HeaderValue::from_str("\0").unwrap_err();
        let err = Error::HeaderError(header_err);
        let display = format!("{}", err);
        assert!(display.starts_with("Invalid header value: "));
    }

    #[test]
    fn test_display_error_responses() {
        let responses = vec![ErrorResponse {
            message: "Record not found".to_string(),
            error_code: "NOT_FOUND".to_string(),
            fields: None,
        }];
        let err = Error::ErrorResponses(responses);
        let display = format!("{}", err);
        assert!(display.contains("Error response from Salesforce"));
        assert!(display.contains("NOT_FOUND"));
    }

    #[test]
    fn test_display_describe_error() {
        let resp = ErrorResponse {
            message: "Cannot describe".to_string(),
            error_code: "INVALID_TYPE".to_string(),
            fields: Some(vec!["Name".to_string()]),
        };
        let err = Error::DescribeError(resp);
        let display = format!("{}", err);
        assert!(display.contains("Error completing describe"));
    }

    #[test]
    fn test_display_login_error() {
        let resp = LoginErrorResponse {
            message: "Login failed".to_string(),
            error_code: "LOGIN_FAILED".to_string(),
        };
        let err = Error::LoginError(resp);
        let display = format!("{}", err);
        assert!(display.contains("Error logging in"));
    }

    #[test]
    fn test_error_is_std_error() {
        let err = Error::NotLoggedIn;
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_source_deserialize_error() {
        let json_err = serde_json::from_str::<String>("bad").unwrap_err();
        let err = Error::DeserializeError(json_err);
        assert!(StdError::source(&err).is_some());
    }

    #[test]
    fn test_source_header_error() {
        let header_err = reqwest::header::HeaderValue::from_str("\0").unwrap_err();
        let err = Error::HeaderError(header_err);
        assert!(StdError::source(&err).is_some());
    }

    #[test]
    fn test_source_none_for_other_variants() {
        assert!(StdError::source(&Error::NotLoggedIn).is_none());
        assert!(StdError::source(&Error::ConfigError("test".to_string())).is_none());
    }

    #[test]
    fn test_from_serde_json_error() {
        let json_err = serde_json::from_str::<String>("invalid json").unwrap_err();
        let err: Error = json_err.into();
        match err {
            Error::DeserializeError(_) => {}
            _ => panic!("Expected DeserializeError"),
        }
    }

    #[test]
    fn test_from_invalid_header_value() {
        let header_err = reqwest::header::HeaderValue::from_str("\0").unwrap_err();
        let err: Error = header_err.into();
        match err {
            Error::HeaderError(_) => {}
            _ => panic!("Expected HeaderError"),
        }
    }

    #[test]
    fn test_debug_impl() {
        let err = Error::NotLoggedIn;
        let debug = format!("{:?}", err);
        assert_eq!(debug, "NotLoggedIn");
    }
}