//! Salesforce API Client
//!
//! This crate provides a client with several authentication types for the Salesforce APIs.
//!

use super::responses::login_error_response::LoginErrorResponse;
use crate::client::responses::access_token::AccessToken;
use crate::client::responses::token_response::TokenResponse;
use crate::client::xml::{create_login_envelope, extract_xml_tag};
use crate::errors::Error;
use log::debug;
use regex::Regex;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderName, HeaderValue};
use reqwest::{Response, Url};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(feature = "metadata-api")]
use reqwest::multipart::Form;


/// Represents a client used for interacting with a remote API.
/// This struct encapsulates all the necessary data required to authenticate
/// and send HTTP requests to the API.
///
/// # Fields
///
/// * `http_client` - The underlying HTTP client (`reqwest::Client`) used to execute requests.
/// * `client_id` - An optional client ID used for authentication.
/// * `client_secret` - An optional client secret used for authentication.
/// * `login_endpoint` - The URL of the login endpoint used for authenticating requests.
/// * `instance_url` - An optional URL of the API instance to which the client is connected.
/// * `access_token` - An optional access token representing the user session for authenticated requests.
/// * `refresh_token` - An optional refresh token used to obtain a new access token when it expires.
/// * `version` - The version of the API being used.
/// * `secret_required` - A boolean value indicating whether the client secret is mandatory for authentication.
///
/// # Derive Attributes
///
/// * `Clone` - Enables creating a deep copy of the `Client`.
/// * `Debug` - Enables formatting a debugging representation of the `Client`.
///
/// # Usage
///
/// This struct is designed to facilitate authenticated communication with an API.
/// Use it to configure connection settings, store authentication tokens,
/// and manage HTTP request creation.
///
/// Example usage may include setting up the client with credentials, defining the
/// API endpoint, and invoking authenticated API calls with the underlying HTTP client.
///
/// # Example
/// ```rust
/// use rustsf::{Client, Error};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     let mut client = Client::new();
///
///     // Configure authentication credentials, e.g.:
///     client.set_client_id("client_Id");
///     client.set_client_secret("client_secret");
///     match client.login_with_credential("username", "password").await {
///         Ok(_) => println!("Login successful!"),
///         Err(e) => println!("Login failed: {}", e),
///     }
///     // ... other logic ...
///     Ok(())
/// }
/// ```
///
/// # Note
///
/// Ensure that sensitive fields like `client_secret` and `access_token` are handled securely
/// to avoid unintended exposure of confidential data.
#[derive(Clone, Debug)]
pub struct Client {
    pub http_client: reqwest::Client,
    pub(crate) client_id: Option<String>,
    pub(crate) client_secret: Option<String>,
    pub(crate) login_endpoint: String,
    pub(crate) instance_url: Option<String>,
    pub access_token: Option<AccessToken>,
    pub(crate) refresh_token: Option<String>,
    pub(crate) version: String,
    pub(crate) secret_required: bool,
    pub(crate) organisation_id: Option<String>,
    pub(crate) user_id: Option<String>,
}

impl Default for Client {
    fn default() -> Self {
        Client::new()
    }
}

impl Client {
    /// Creates a new `Client` instance with default configuration.
    ///
    /// This function initializes a `Client` with the following default properties:
    /// - A new `reqwest::Client` instance for making HTTP requests.
    /// - `client_id`: `None`
    /// - `client_secret`: `None`
    /// - `login_endpoint`: Set to `"https://login.salesforce.com"`.
    /// - `access_token`: `None`
    /// - `instance_url`: `None`
    /// - `refresh_token`: `None`
    /// - `secret_required`: Set to `true`.
    /// - `version`: Set to `"v60.0"`.
    ///
    /// # Returns
    /// A configured `Client` instance ready for use.
    ///
    /// # Examples
    /// ```
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     Ok(())
    /// }
    /// ```
    pub fn new() -> Client {
        let http_client = reqwest::Client::new();
        Client {
            http_client,
            client_id: None,
            client_secret: None,
            login_endpoint: "https://login.salesforce.com".to_string(),
            access_token: None,
            instance_url: None,
            refresh_token: None,
            secret_required: true,
            version: "v60.0".to_string(),
            organisation_id: None,
            user_id: None,
        }
    }

    // --- Read-only getters ---

    /// Retrieves the client ID associated with the current object, if it exists.
    ///
    /// # Returns
    /// - `Option<&str>`:
    ///    - `Some(&str)` containing the reference to the client ID if it is present.
    ///    - `None` if the client ID is not set.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // ... set client ID and other fields ...
    ///     let client_id = client.client_id();
    ///     Ok(())
    /// }
    /// ```
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
    /// # Example
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let client_id = client.client_secret();
    ///     Ok(())
    /// }
    /// ```
    pub fn client_secret(&self) -> Option<&str> {
        self.client_secret.as_deref()
    }

    /// Returns the login endpoint URL as a string slice.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let endpoint = client.login_endpoint();
    ///     assert_eq!(endpoint, "https://login.salesforce.com");
    ///     Ok(())
    /// }
    /// ```
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
    /// # Examples
    ///
    /// ```
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let endpoint = client.instance_url();
    ///     Ok(())
    /// }
    /// ```
    pub fn instance_url(&self) -> Option<&str> {
        self.instance_url.as_deref()
    }

    pub fn validated_base_url(&self) -> Result<String, Error> {
        let instance_url = self.instance_url.as_ref().ok_or(Error::NotLoggedIn)?;
        Ok(format!("{}/services/data/", instance_url))
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
    /// # Examples
    ///
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let token = client.access_token();
    ///     Ok(())
    /// }
    /// ```
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
    /// # Example
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let token_value = client.access_token_value();
    ///     Ok(())
    /// }
    /// ```
    pub fn access_token_value(&self) -> Option<&str> {
        self.access_token.as_ref().map(|t| t.value.as_str())
    }

    /// Returns the version of the current instance.
    ///
    /// # Returns
    /// A string slice that holds the version information.
    ///
    /// # Examples
    /// ```
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     assert_eq!(client.version(), "v60.0");
    ///     Ok(())
    /// }
    /// ```
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Returns the bare version of the current instance. (without the 'v' prefix)
    ///
    /// # Returns
    /// A string that holds the version information.
    ///
    /// # Examples
    /// ```
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     assert_eq!(client.version(), "v60.0");
    ///     Ok(())
    /// }
    /// ```
    pub fn version_number(&self) -> Result<String, Error> {
        let mut chars = self.version.chars();
        chars.next();
        Ok(chars.as_str().to_string())  // fixme might want to store just the base version number and then add the "v" when really needed...
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
    /// # Example
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let token = client.refresh_token();
    ///     Ok(())
    /// }
    /// ```
    pub fn refresh_token(&self) -> Option<&str> {
        self.refresh_token.as_deref()
    }

    /// Constructs the base path URL for the API, incorporating the instance URL and version.
    ///
    /// # Returns
    ///
    /// - `Ok(String)` containing the formatted base path URL if the `instance_url` is available.
    /// - `Err(Error::NotLoggedIn)` if the `instance_url` is not set.
    ///
    /// # Errors
    ///
    /// Returns an error of type `Error::NotLoggedIn` if the user is not logged in and the `instance_url`
    /// is unavailable.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     match client.base_version_path() {
    ///         Ok(base_path) => println!("Base path: {}", base_path),
    ///         Err(e) => println!("Error: {}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This method assumes that `version` is a valid string and does not perform validation on it.
    pub fn base_version_path(&self) -> Result<String, Error> {
        let instance_url = self.instance_url.as_ref().ok_or(Error::NotLoggedIn)?;
        Ok(format!("{}/services/data/{}", instance_url, self.version))
    }

    pub fn base_path(&self) -> Result<String, Error> {
        let instance_url = self.instance_url.as_ref().ok_or(Error::NotLoggedIn)?;
        Ok(format!("{}/services/data/", instance_url))
    }

    pub fn soap_path(&self) -> Result<String, Error> {
        let instance_url = self.instance_url.as_ref().ok_or(Error::NotLoggedIn)?;
        Ok(format!("{}/services/Soap/m/", instance_url))
    }

    pub fn soap_version_path(&self) -> Result<String, Error> {
        let instance_url = self.instance_url.as_ref().ok_or(Error::NotLoggedIn)?;
        Ok(format!("{}/services/Soap/m/{}", instance_url, self.version_number()?))
    }



    // --- Setters ---

    /// Sets the login endpoint for the current instance.
    ///
    /// This method allows you to specify a custom login endpoint URL
    /// for the object. It internally converts the provided string slice (`&str`)
    /// into a `String` and assigns it to the `login_endpoint` field.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - A string slice that specifies the login endpoint URL to be set.
    ///
    /// # Returns
    ///
    /// A mutable reference to the current instance, allowing method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     client.set_login_endpoint("https://test.salesforce.com/login");
    ///     // rest of authentication logic...
    ///     Ok(())
    /// }
    /// ```
    pub fn set_login_endpoint(&mut self, endpoint: &str) -> &mut Self {
        self.login_endpoint = endpoint.to_string();
        self
    }

    /// Sets the version for the current instance.
    ///
    /// This method updates the `version` field of the struct to the provided string.
    /// It consumes a string slice, converts it to a `String`, and assigns it to the
    /// instance's `version` field.
    ///
    /// # Parameters
    /// - `version`: A string slice representing the new version to be set.
    ///
    /// # Returns
    /// A mutable reference to the current instance, allowing for method chaining.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     client.set_version("v65.0");
    ///     Ok(())
    /// }
    /// ```
    pub fn set_version(&mut self, version: &str) -> &mut Self {
        self.version = version.to_string();
        self
    }

    /// Sets the instance URL for the current object.
    ///
    /// This method allows you to set the `instance_url` field of the object to the specified
    /// URL string. The provided URL is converted into an owned `String` and stored inside
    /// the object. The method then returns a mutable reference to the current object,
    /// allowing for method chaining.
    ///
    /// # Arguments
    ///
    /// * `instance_url` - A string slice that represents the URL to be set.
    ///
    /// # Returns
    ///
    /// A mutable reference to the current object (`&mut Self`) to enable method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     client.set_instance_url("https://develop.sandbox.mu.salesforce.com");
    ///     // Authentication logic...
    ///     Ok(())
    /// }
    /// ```
    pub fn set_instance_url(&mut self, instance_url: &str) -> &mut Self {
        self.instance_url = Some(instance_url.to_string());
        self
    }

    /// Sets the refresh token for the current instance.
    ///
    /// This method takes a string slice representing a refresh token,
    /// converts it to a `String`, and assigns it to the `refresh_token` field
    /// of the struct. After setting the value, it returns a mutable reference
    /// to `self`, allowing method chaining.
    ///
    /// # Arguments
    ///
    /// * `refresh_token` - A string slice representing the refresh token to be set.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - A mutable reference to the current instance, enabling method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     client.set_refresh_token("new_refresh_token");
    ///     Ok(())
    /// }
    /// ```
    pub fn set_refresh_token(&mut self, refresh_token: &str) -> &mut Self {
        self.refresh_token = Some(refresh_token.to_string());
        self
    }

    /// Sets whether a secret is required for the entity and returns a mutable reference to the instance.
    ///
    /// # Parameters
    /// - `secret_required`: A boolean value indicating whether the entity requires a secret.
    ///    - `true`: A secret is required.
    ///    - `false`: A secret is not required.
    ///
    /// # Returns
    /// A mutable reference to the current instance of the object, enabling method chaining.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     client.set_secret_required(true);
    ///     Ok(())
    /// }
    /// ```
    pub fn set_secret_required(&mut self, secret_required: bool) -> &mut Self {
        self.secret_required = secret_required;
        self
    }

    /// Sets the client ID for the instance.
    ///
    /// This method allows you to assign a client ID to the current instance.
    /// The provided string reference is converted into an owned `String` and
    /// stored internally. It returns a mutable reference to the instance,
    /// allowing for method chaining.
    ///
    /// # Arguments
    ///
    /// * `client_id` - A string slice that holds the client ID to be set.
    ///
    /// # Returns
    ///
    /// * A mutable reference to the current instance (`Self`), enabling method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     client.set_client_id("client_id");
    ///     Ok(())
    /// }
    ///     // Rest of authentication logic...
    /// ```
    pub fn set_client_id(&mut self, client_id: &str) -> &mut Self {
        self.client_id = Some(client_id.to_string());
        self
    }

    /// Sets the client secret for the current instance.
    ///
    /// This method allows you to set the client secret, which is typically
    /// used for authentication purposes or identifying the client in API requests.
    ///
    /// # Arguments
    ///
    /// * `client_secret` - A string slice containing the client secret to be set.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the current instance, allowing method
    /// chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     client.set_client_secret("client_secret");
    ///     Ok(())
    /// }
    /// ```
    pub fn set_client_secret(&mut self, client_secret: &str) -> &mut Self {
        self.client_secret = Some(client_secret.to_string());
        self
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
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     client.set_access_token("abc123".to_string(), "2023-01-01T00:00:00Z".to_string(), "Bearer".to_string());
    ///     Ok(())
    /// }
    /// ```
    ///
    /// This sets the access token, token type, and issuance time for the client.
    pub fn set_access_token(
        &mut self,
        access_token: String,
        issued_at: String,
        token_type: String,
    ) -> &mut Self {
        self.access_token = Some(AccessToken {
            token_type,
            value: access_token,
            issued_at,
        });
        self
    }

    /// Asynchronously retrieves the identity from the provided URL.
    ///
    /// This function sends a GET request to the specified `identity_url`
    /// without any additional headers. If the response has a successful
    /// status code, the response body is returned as a `String`. Otherwise,
    /// an error is returned, containing a detailed description of the
    /// failure.
    ///
    /// # Arguments
    ///
    /// * `identity_url` - A `String` containing the URL to fetch the identity from.
    ///
    /// # Returns
    ///
    /// * `Result<String, Error>` - Returns `Ok(String)` containing the response body if the
    ///   request was successful, or `Err(Error)` if an error occurred.
    ///
    /// # Errors
    ///
    /// This function returns an error in these cases:
    /// - If the HTTP GET request fails.
    /// - If the response status is not successful.
    /// - If there is an issue reading or parsing the response body.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let identity_url = client.get_identity("identity_url".to_string()).await;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Dependencies
    ///
    /// This function depends on the `get` method of the client object and assumes that:
    /// * The `get` method takes a URL and headers as parameters.
    /// * The `res` object has `status`, `text`, and `json` methods, where:
    ///   - `status()` returns the status of the response.
    ///   - `text().await` retrieves the response body as a `String`.
    ///   - `json().await` deserializes the response body into an error description.
    pub async fn get_identity(&mut self, identity_url: String) -> Result<String, Error> {
        let res = self.get(identity_url, vec![], vec![]).await?;
        if res.status().is_success() {
            Ok(res.text().await?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    pub fn get_user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }

    /// Asynchronously ensures that the access token is refreshed if it has expired or
    /// if it cannot be parsed for comparison.
    ///
    /// # Behavior
    /// - If `self.access_token` is `None`, this function does nothing and returns `Ok(self)`.
    /// - If the `issued_at` timestamp of the `access_token` cannot be parsed as a Unix
    ///   timestamp (milliseconds since epoch), the function will attempt to refresh
    ///   the access token immediately.
    /// - If the `issued_at` timestamp is successfully parsed and the corresponding access token
    ///   is still valid (within 2 hours of being issued), no action is taken.
    /// - If the access token has expired (2 hours past the `issued_at` time), the function
    ///   will attempt to refresh the token.
    ///
    /// # Returns
    /// - `Ok(&mut Self)` if the access token is valid or successfully refreshed.
    /// - `Err(Error)` if there is an error during the refresh process.
    ///
    /// # Logs
    /// - Logs an informational message if the `issued_at` timestamp cannot be parsed.
    /// - Logs an informational message if the access token has expired and a refresh is attempted.
    ///
    /// # Errors
    /// Returns an `Error` if the refresh operation fails.
    ///
    /// # Examples
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// async fn example() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     client.ensure_refresh().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn ensure_refresh(&mut self) -> Result<&mut Self, Error> {
        if self.access_token.is_none() {
            return Ok(self);
        }

        let issued_at = &self.access_token.as_ref().unwrap().issued_at;
        let timestamp_ms = match issued_at.parse::<u64>() {
            Ok(ts) => ts,
            Err(_) => {
                // SOAP login returns ISO timestamp; can't compare, attempt refresh
                log::info!("Could not parse issued_at as timestamp, attempting refresh.");
                return self.refresh().await;
            }
        };
        let seconds = timestamp_ms / 1000;
        let nanos = (timestamp_ms % 1000) * 1_000_000;

        let given_time = UNIX_EPOCH + Duration::new(seconds, nanos as u32);

        let two_hours = Duration::from_secs(2 * 60 * 60); // 2 hours in seconds
        let modified_time = given_time + two_hours;

        let current_time = SystemTime::now();

        if current_time > modified_time {
            log::info!("Access Token Expired, Refreshing.");
            Ok(self.refresh().await?)
        } else {
            Ok(self)
        }
    }

    /// Generates a vector of key-value pairs representing parameters
    /// required for a token refresh request.
    ///
    /// # Returns
    ///
    /// A `Vec` of tuples where each tuple contains:
    /// - A `String` representing the parameter name.
    /// - A `String` representing the parameter value.
    ///
    /// The parameters include:
    /// - `"grant_type"`: Always set to `"refresh_token"`.
    /// - `"refresh_token"`: The `refresh_token` value (empty if not provided).
    /// - `"client_id"`: The `client_id` value (empty if not provided).
    ///
    /// If the `secret_required` field is `true`, the following parameter is also included:
    /// - `"client_secret"`: The `client_secret` value (empty if not provided).
    ///
    /// # Example
    ///
    /// ```rust, ignore
    /// let params = client.get_refresh_params();
    /// for (key, value) in params {
    ///     println!("{}: {}", key, value);
    /// }
    /// ```
    ///
    /// This function ensures that default values are used if any of the required
    /// fields (`refresh_token`, `client_id`, `client_secret`) are not set.
    fn get_refresh_params(&self) -> Vec<(String, String)> {
        let refresh_token = self.refresh_token.clone().unwrap_or_default();
        let client_id = self.client_id.clone().unwrap_or_default();

        let mut params = vec![
            ("grant_type".to_string(), "refresh_token".to_string()),
            ("refresh_token".to_string(), refresh_token),
            ("client_id".to_string(), client_id),
        ];

        if self.secret_required {
            params.push((
                "client_secret".to_string(),
                self.client_secret.clone().unwrap_or_default(),
            ));
        }
        params
    }

    /// Refreshes the authentication token using the refresh token flow.
    ///
    /// This asynchronous function performs the following steps:
    /// 1. Constructs a token endpoint URL using the `login_endpoint`.
    /// 2. Retrieves the required parameters for token refresh using `get_refresh_params()`.
    /// 3. Sends an HTTP POST request with the parameters to the token endpoint.
    /// 4. Processes the response:
    ///    - If the HTTP request fails, an error is returned.
    ///    - If the response status is not successful, it attempts to parse
    ///      and return a token-specific error.
    ///    - If the response is successful, it parses the token response,
    ///      updates the current access token, sets the token issuance time,
    ///      token type, and updates the instance URL.
    ///
    /// # Returns
    /// - `Ok(&mut Self)` if the token has been successfully refreshed.
    /// - `Err(Error)` if the token refresh fails due to an HTTP error,
    ///   a server-side error, or an unexpected response structure.
    ///
    /// # Errors
    /// Returns an `Error` in the following cases:
    /// - HTTP transmission error during the token refresh request.
    /// - Unsuccessful HTTP response due to invalid credentials or other
    ///   server-side errors.
    /// - Failure to parse the error or token response JSON into the
    ///   appropriate data structures.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     match client.refresh().await {
    ///         Ok(updated_client) => {
    ///          println!("Token refreshed successfully!");
    ///         },
    ///         Err(e) => {
    ///             eprintln!("Failed to refresh token: {:?}", e);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Dependencies
    /// - This function expects that the struct has an `http_client` field
    ///   capable of making asynchronous HTTP requests (e.g., an instance of `reqwest::Client`).
    /// - The function assumes the existence of `get_refresh_params`, `set_access_token`,
    ///   and other utility methods within the struct to handle token management.
    pub async fn refresh(&mut self) -> Result<&mut Self, Error> {

        /// Extracts the Salesforce organization id and user idL.
        ///
        /// Example: `https://login.salesforce.com/id/00D50000000IZ3ZEAW/00550000001fg5OAAQ`
        fn extract_organisation_and_user_id(url: &str) -> Option<(String, String)> {
            let path = url.split_once("/id/")?.1;
            let mut parts = path.split('/');

            let organisation_id = parts.next()?.trim();
            let user_id = parts.next()?.trim();

            if organisation_id.is_empty() || user_id.is_empty() {
                return None;
            }

            Some((organisation_id.to_string(), user_id.to_string()))
        }

        let token_url = format!("{}/services/oauth2/token", self.login_endpoint);
        let params = self.get_refresh_params();

        let res = self
            .http_client
            .post(token_url.as_str())
            .form(&params)
            .send()
            .await;

        let res = match res {
            Ok(res) => res,
            Err(e) => return Err(e.into()),
        };

        if !res.status().is_success() {
            let error_response = res.json().await?;
            return Err(Error::TokenError(error_response));
        }

        let response: TokenResponse = res.json().await?;
        let token_type = response.token_type.unwrap_or_default();
        self.set_access_token(response.access_token, response.issued_at, token_type);
        self.instance_url = Some(response.instance_url);

        match extract_organisation_and_user_id(response.id.as_str()) {
            Some((org_id, user_id)) => {
                self.organisation_id = Some(org_id);
                self.user_id = Some(user_id);
            }
            None => {
                log::warn!("Could not extract organisation and user ID from token response.");
            }
        }

        Ok(self)
    }

    /// Logs in using the provided username and password credentials,
    /// retrieves an access token from the OAuth2 token endpoint,
    /// and updates the internal client state.
    ///
    /// # Arguments
    ///
    /// - `username`: A string slice containing the user's username.
    /// - `password`: A string slice containing the user's password.
    ///
    /// # Returns
    ///
    /// Returns:
    /// - `Ok(&mut Self)` if the login and token retrieval are successful, allowing method chaining.
    /// - `Err(Error)` if there are any configuration issues, HTTP request errors, or unsuccessful responses.
    ///
    /// # Errors
    ///
    /// The function can return the following errors:
    /// - `ConfigError`: If `client_id` or `client_secret` is not configured.
    /// - `HttpError`: If there is an issue with the HTTP request, such as network failure.
    /// - `TokenError`: If the server returns an unsuccessful response, e.g., invalid credentials.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     client.set_client_id("example_client_id");
    ///     client.set_client_secret("example_client_secret");
    ///
    ///     let result = client.login_with_credential("username", "password").await;
    ///     match result {
    ///         Ok(client) => println!("Login successful!"),
    ///         Err(e) => eprintln!("Login failed: {:?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Behavior
    ///
    /// - Constructs a token request URL based on the configured `login_endpoint`.
    /// - Validates the presence of `client_id` and `client_secret` configurations.
    /// - Sends an HTTP POST request with the required parameters to fetch an access token.
    /// - Upon a successful response:
    ///     - Updates the internal client state with the access token, token type, issue timestamp, and instance URL.
    /// - On error, the function returns a descriptive `Error` for troubleshooting.
    ///
    /// # Notes
    ///
    /// - Ensure that the `client_id` and `client_secret` are set before calling this method.
    /// - The function uses the `reqwest` library for HTTP requests and assumes the `http_client` is properly initialized.
    pub async fn login_with_credential(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<&mut Self, Error> {
        let token_url = format!("{}/services/oauth2/token", self.login_endpoint);
        let client_id = self
            .client_id
            .as_ref()
            .ok_or_else(|| Error::ConfigError("client_id is required".to_string()))?;
        let client_secret = self
            .client_secret
            .as_ref()
            .ok_or_else(|| Error::ConfigError("client_secret is required".to_string()))?;
        let params = [
            ("grant_type", "password"),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("username", username),
            ("password", password),
        ];
        let res = self
            .http_client
            .post(token_url.as_str())
            .form(&params)
            .send()
            .await?;

        if !(res.status().is_success()) {
            let error_response = res.json().await?;
            return Err(Error::TokenError(error_response));
        }

        let response: TokenResponse = res.json().await?;
        let token_type = response.token_type.unwrap_or_default();
        self.set_access_token(response.access_token, response.issued_at, token_type);
        self.instance_url = Some(response.instance_url);
        Ok(self)
    }

    /// Asynchronously log in to Salesforce using an SFDX Auth URL.
    ///
    /// This method allows the user to authenticate with Salesforce by parsing an SFDX authentication URL,
    /// extracting the necessary credentials, and exchanging the refresh token for an access token.
    ///
    /// # Parameters
    /// - `sfdx_auth_url`: A `&str` containing the SFDX authentication URL. The format of the auth URL must
    ///   match the pattern:
    ///   `force://<client_id>:<client_secret>:<refresh_token>@<login_endpoint>`.
    ///
    /// # Returns
    /// - On success, this function returns a mutable reference to the `Self` instance wrapped in a
    ///   `Result::Ok`. The `Self` instance will have the `access_token` and `instance_url` fields updated
    ///   with the authentication response.
    /// - On failure, an `Error` is returned, which can indicate issues like errors during the token fetch
    ///   process or invalid credentials.
    ///
    /// # Errors
    /// - Returns `Error::NotLoggedIn` if the response from Salesforce does not include a valid `token_type`.
    /// - Returns `Error::TokenError` if the Salesforce token service responds with an error payload.
    /// - Returns other `Error` variants if the HTTP request fails or is invalid.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let sfdx_auth_url = "force://your_client_id:your_secret_token:your_refresh_token@your.login.endpoint";
    ///     let mut client = Client::new();
    ///
    ///     match client.login_with_sfdx_auth_url(sfdx_auth_url).await {
    ///         Ok(_) => {
    ///             println!("Successfully logged in!");
    ///         },
    ///         Err(e) => {
    ///             eprintln!("Failed to log in: {:?}", e);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Details
    /// - The function utilizes a regular expression to extract credentials from the `sfdx_auth_url`.
    /// - It uses an HTTP POST request to exchange the refresh token for an access token via Salesforce's
    ///   OAuth2 token endpoint.
    /// - The response, if successful, updates the following fields of the `Self` instance:
    ///   - `access_token`: Contains the access token and associated metadata.
    ///   - `instance_url`: Contains the base URL of the Salesforce instance.
    ///
    /// # Dependencies
    /// - This function uses the `regex` crate for regular expression handling.
    /// - The `reqwest` crate is used for making the HTTP POST request.
    ///
    /// # Notes
    /// - Ensure that the `sfdx_auth_url` provided is valid and has the expected format.
    /// - This function relies on async/await for non-blocking execution. It must be used within an
    ///   async runtime.
    pub async fn login_with_sfdx_auth_url(
        &mut self,
        sfdx_auth_url: &str,
    ) -> Result<&mut Self, Error> {
        let re = Regex::new(r"force://([a-zA-Z0-9._-]+):([a-zA-Z0-9._-]*):([a-zA-Z0-9._-]+={0,2})@([a-zA-Z0-9._-]+)").unwrap();
        let caps = re.captures(&sfdx_auth_url).unwrap();

        self.set_client_id(&caps[1]);
        self.set_client_secret(&caps[2]);
        self.set_refresh_token(&caps[3]);
        self.set_login_endpoint(&caps[4]);

        let token_url = format!("https://{}/services/oauth2/token", self.login_endpoint);
        println!("token_url: {}", token_url);
        let params = [
            ("grant_type", "refresh_token"),
            ("client_id", self.client_id.as_ref().unwrap()),
            ("refresh_token", self.refresh_token.as_ref().unwrap()),
        ];
        let res = self
            .http_client
            .post(token_url.as_str())
            .form(&params)
            .send()
            .await?;

        if res.status().is_success() {
            let r: TokenResponse = res.json().await?;
            self.access_token = Some(AccessToken {
                value: r.access_token,
                issued_at: r.issued_at,
                token_type: r.token_type.ok_or(Error::NotLoggedIn)?,
            });
            self.instance_url = Some(r.instance_url);

            match self.extract_organisation_and_user_id(r.id.as_str()) {
                Some((org_id, user_id)) => {
                    self.organisation_id = Some(org_id);
                    self.user_id = Some(user_id);
                }
                None => {
                    log::warn!("Could not extract organisation and user ID from token response.");
                }
            }

            Ok(self)
        } else {
            let error_response = res.json().await?;
            Err(Error::TokenError(error_response))
        }
    }

    fn extract_organisation_and_user_id(&self, url: &str) -> Option<(String, String)> {
        let path = url.split_once("/id/")?.1;
        let mut parts = path.split('/');

        let organisation_id = parts.next()?.trim();
        let user_id = parts.next()?.trim();

        if organisation_id.is_empty() || user_id.is_empty() {
            return None;
        }

        Some((organisation_id.to_string(), user_id.to_string()))
    }

    /// Asynchronously performs a login process using a SOAP API and updates the client instance
    /// with the retrieved access token and instance URL.
    ///
    /// # Arguments
    ///
    /// - `username`: A reference to a string slice representing the username
    ///   to authenticate with.
    /// - `password`: A reference to a string slice representing the password
    ///   associated with the username.
    ///
    /// # Returns
    ///
    /// Returns a `Result`:
    /// - On success, it returns `Ok(&mut Self)`, where the instance is updated with
    ///   the `access_token` and `instance_url` obtained from the SOAP response.
    /// - On failure, it returns `Err(Error)`, where `Error` contains information about
    ///   the login error, such as the error message and error code.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The HTTP request fails.
    /// - The SOAP API responds with an unsuccessful status code.
    /// - The expected XML tags (`sessionId`, `serverTimestamp`, `serverUrl`, etc.)
    ///   are not present in the response.
    ///
    /// # SOAP API Details
    ///
    /// - The SOAP endpoint URL is constructed using the `self.login_endpoint`
    ///   and `self.version`, formatted as `{login_endpoint}/services/Soap/u/{version}`.
    /// - The SOAP envelope containing the login request is generated by the
    ///   `create_login_envelope` function.
    /// - Relevant details such as the session token (`sessionId`) and instance URL
    ///   (`serverUrl`) are extracted from the SOAP response using the `extract_xml_tag`
    ///   utility.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     let result = client.login_with_soap("username", "password").await;
    ///     match result {
    ///         Ok(updated_client) => {
    ///             println!("Login successful.");
    ///             println!("Access Token: {:?}", updated_client.access_token);
    ///             println!("Instance URL: {:?}", updated_client.instance_url);
    ///         }
    ///         Err(e) => {
    ///             eprintln!("Login failed: {:?}", e);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - Ensure that the input SOAP envelope and the response XML structure
    ///   conform to the expected SOAP API specification.
    /// - If the `access_token` cannot be extracted from the response,
    ///   the `access_token` field will remain `None` in the updated client instance.
    ///
    /// # Dependencies
    ///
    /// - An HTTP client that supports asynchronous operations is required, e.g.,
    ///   `reqwest`.
    /// - XML parsing utilities (e.g., `extract_xml_tag`) must be able to handle
    ///   the relevant tags properly in the SOAP response.
    ///
    /// # Associated Types
    ///
    /// - This function leverages an `AccessToken` struct, which contains
    ///   fields like `value` (session token), `issued_at` (server timestamp),
    ///   and `token_type` ("Bearer" by default).
    ///
    /// # See Also
    ///
    /// - `create_login_envelope` for generating the required SOAP envelope.
    ///
    /// - `extract_xml_tag` for parsing specific XML tags from the response.
    ///
    /// - `Error::LoginError` for understanding the structure of login errors.
    ///
    /// # Deprecated
    /// The use of login by SOAP is deprecated by Salesforce and should be avoided
    #[deprecated(
        since = "0.0.1",
        note = "The use of login by SOAP is deprecated by Salesforce and should be avoided"
    )]
    pub async fn login_with_soap(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<&mut Self, Error> {
        let token_url = format!("{}/services/Soap/u/{}", self.login_endpoint, self.version);
        let body = create_login_envelope(username, password);
        let res = self
            .http_client
            .post(token_url.as_str())
            .body(body)
            .header("Content-Type", "text/xml")
            .header("SOAPAction", "\"\"")
            .send()
            .await?;
        if res.status().is_success() {
            let body_response = res.text().await?;
            self.access_token = match extract_xml_tag("sessionId", body_response.as_str()) {
                Some(t) => {
                    let issued_at = extract_xml_tag("serverTimestamp", body_response.as_str())
                        .unwrap_or_default();
                    Some(AccessToken {
                        value: t,
                        issued_at,
                        token_type: "Bearer".to_string(),
                    })
                }
                None => None,
            };
            self.instance_url = extract_xml_tag("serverUrl", body_response.as_str());
            Ok(self)
        } else {
            let body_response = res.text().await?;
            let error_message =
                extract_xml_tag("faultstring", body_response.as_str()).unwrap_or_default();
            let error_code =
                extract_xml_tag("faultcode", body_response.as_str()).unwrap_or_default();

            Err(Error::LoginError(LoginErrorResponse {
                message: error_message,
                error_code,
            }))
        }
    }

    /// Asynchronously performs an HTTP GET request to a specified Apex REST endpoint using a full URI.
    ///
    /// # Parameters
    /// - `uri`: A string slice representing the relative path or endpoint of the resource to be accessed.
    ///
    /// # Returns
    /// - `Result<Response, Error>`:
    ///   - On success, returns a `Response` containing the result of the GET request.
    ///   - On failure, returns an `Error` indicating the reason for failure:
    ///     - `Error::NotLoggedIn`: If the `instance_url` field is not available (user is not logged in).
    ///     - `Error::ConfigError`: If the generated URL for the request is invalid.
    ///
    /// # Description
    /// This function constructs the full URL for the requested resource by combining the base
    /// `instance_url` (from the current state) with the provided `uri` parameter. It validates
    /// the resulting URL and extracts query parameters for the request. The function then delegates
    /// the constructed path and parameters to the `rest_get` method, which executes the actual GET request.
    ///
    /// The following steps are performed:
    /// 1. Combine the base `instance_url` and `uri` to generate the full resource URL.
    /// 2. Parse the URL to verify its validity and extract query parameters.
    /// 3. Convert query parameters into suitable format for internal processing.
    /// 4. Extract the URL path and invoke `rest_get` with the path and query parameters.
    ///
    /// # Errors
    /// - Returns `Error::NotLoggedIn` if no `instance_url` is available in the current context.
    /// - Returns `Error::ConfigError` if URL parsing fails due to an invalid format.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     match client.rest_get_fulluri("my/resource/endpoint").await {
    ///         Ok(response) =>  println!("Response: {:?}", response),
    ///         Err(e) => println!("Error: {:?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    /// The function assumes that the `self.rest_get` method is implemented to handle the GET request
    /// based on the extracted path and parameters.
    pub async fn rest_get_fulluri(&mut self, uri: &str) -> Result<Response, Error> {
        let resource_url = format!(
            "{}/services/apexrest/{}",
            self.instance_url.as_ref().ok_or(Error::NotLoggedIn)?,
            uri
        );
        let parsed = Url::parse(&resource_url)
            .map_err(|e| Error::ConfigError(format!("Invalid URL: {}", e)))?;
        // Some ownership absurdity for string refs accessed through iterators with collect
        let hash_query: HashMap<_, _> = parsed.query_pairs().into_owned().collect();
        let params_string: Vec<(String, String)> = hash_query
            .keys()
            .map(|k| (String::from(k), String::from(&hash_query[k])))
            .collect();
        let params: Vec<(&str, &str)> = params_string
            .iter()
            .map(|(x, y)| (&x[..], &y[..]))
            .collect();
        let path: String = parsed.path().to_string();
        self.rest_get(path, params).await
    }

    /// Asynchronously performs an HTTP GET request to a specified REST API endpoint.
    ///
    /// # Arguments
    ///
    /// * `path` - A `String` representing the relative path of the endpoint to be queried on the server.
    /// * `params` - A `Vec` of key-value tuples (`&str`, `&str`) representing query parameters to be appended to the URL.
    ///
    /// # Returns
    ///
    /// * `Result<Response, Error>` -
    ///   - On success: Returns an `Ok(Response)` containing the HTTP response.
    ///   - On failure: Returns an `Err(Error)` indicating the reason for failure.
    ///
    /// # Errors
    ///
    /// This function may return errors in the following scenarios:
    /// * `Error::NotLoggedIn` - If the `instance_url` is not set, indicating the user is not logged in.
    /// * Errors originating from HTTP client operations, such as issues while creating headers, sending the request, or ensuring token refresh.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let path = String::from("/api/v1/data");
    ///     let params = vec![("key", "value"), ("filter", "recent")];
    ///     let response = client.rest_get(path, params).await;
    ///
    ///     match response {
    ///         Ok(res) => {
    ///             println!("Response received: {:?}", res);
    ///         }
    ///         Err(e) => {
    ///             eprintln!("Error occurred: {:?}", e);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// This function ensures that the client's authentication tokens are refreshed before sending the request.
    /// It constructs the full URL by concatenating the `instance_url` and the provided `path`. Query parameters are appended to the URL as needed.
    ///
    /// This function assumes you have an internal configuration for `http_client` and a means to handle HTTP headers through `create_header`.
    pub async fn rest_get(
        &mut self,
        path: String,
        params: Vec<(&str, &str)>,
    ) -> Result<Response, Error> {
        self.ensure_refresh().await?;

        let url = format!(
            "{}{}",
            self.instance_url.as_ref().ok_or(Error::NotLoggedIn)?,
            path
        );
        let res = self
            .http_client
            .get(url.as_str())
            .headers(self.create_header(vec![])?)
            .query(&params)
            .send()
            .await?;
        Ok(res)
    }

    /// Sends a POST request to the specified REST API endpoint with the provided parameters.
    ///
    /// # Type Parameters
    /// * `T`: The type of the request parameters. Must implement the `Serialize` trait.
    ///
    /// # Parameters
    /// * `path`: A `String` specifying the API endpoint relative to the base instance URL.
    /// * `params`: An object of type `T` containing the data to be serialized as JSON and sent
    ///   in the POST request body.
    ///
    /// # Returns
    /// Returns a `Result<Response, Error>` that represents either:
    /// * `Ok(Response)` - The successful HTTP response.
    /// * `Err(Error)` - An error encountered during the process (e.g., not logged in,
    ///   serialization issues, network errors, etc.).
    ///
    /// # Errors
    /// This function can return the following errors:
    /// * `Error::NotLoggedIn` - If the client is not logged in or the instance URL is missing.
    /// * Other errors may be returned due to HTTP request failures or issues during request setup.
    ///
    /// # Behavior
    /// 1. Ensures the client session is refreshed by calling `ensure_refresh()` asynchronously.
    /// 2. Constructs the full URL by appending the given `path` to the instance URL.
    /// 3. Sets the appropriate headers using `create_header`.
    /// 4. Serializes the `params` into JSON.
    /// 5. Sends an asynchronous POST request to the constructed URL.
    /// 6. Returns the received HTTP response upon success.
    ///
    /// # Examples
    /// ```rust
    /// use serde::Serialize;
    /// use rustsf::{Client, Error};
    ///
    /// #[derive(Serialize)]
    /// struct Params {
    ///     key: String,
    ///     value: i32,
    /// }
    ///
    /// async fn example() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication...
    ///     let params = Params {
    ///         key: "example".into(),
    ///         value: 42,
    ///     };
    ///     let response = client
    ///         .rest_post("/api/endpoint".to_string(), params)
    ///         .await?;
    ///
    ///     println!("Response: {:?}", response);
    ///     Ok(())
    /// }
    /// ```
    pub async fn rest_post<T: Serialize>(
        &mut self,
        path: String,
        params: T,
    ) -> Result<Response, Error> {
        self.ensure_refresh().await?;

        let url = format!(
            "{}{}",
            self.instance_url.as_ref().ok_or(Error::NotLoggedIn)?,
            path
        );
        let res = self
            .http_client
            .post(url)
            .headers(self.create_header(vec![])?)
            .json(&params)
            .send()
            .await?;
        Ok(res)
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
    /// - If the `ensure_refresh` internal function fails (e.g., unable to refresh authentication tokens).
    /// - If there is an error serializing the parameters or constructing the HTTP headers.
    /// - If the HTTP request fails during transmission or response handling.
    ///
    /// # Examples
    /// ```rust
    /// use serde::Serialize;
    /// use rustsf::{Client, Error};
    ///
    /// #[derive(Serialize)]
    /// struct UpdateParams {
    ///     field: String,
    ///     value: String,
    /// }
    ///
    /// async fn example() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication...
    ///     let params = UpdateParams {
    ///         field: "example_field".to_string(),
    ///         value: "new_value".to_string(),
    ///     };
    ///
    ///     let response = client
    ///         .rest_patch("/api/endpoint".to_string(), params)
    ///         .await?;
    ///
    ///     println!("Response: {:?}", response);
    ///     Ok(())
    /// }
    /// ```
    pub async fn rest_patch<T: Serialize>(
        &mut self,
        path: String,
        params: T,
    ) -> Result<Response, Error> {
        self.ensure_refresh().await?;

        let url = format!(
            "{}{}",
            self.instance_url.as_ref().ok_or(Error::NotLoggedIn)?,
            path
        );
        let res = self
            .http_client
            .patch(url.as_str())
            .headers(self.create_header(vec![])?)
            .json(&params)
            .send()
            .await?;
        Ok(res)
    }

    /// Sends an HTTP PUT request to the specified REST API endpoint with the given parameters in JSON format.
    ///
    /// This asynchronous function constructs the full URL by appending the provided `path` to the
    /// instance URL, ensures the client is authenticated (by refreshing the session if necessary),
    /// and makes the PUT request using the provided parameters. The function expects the parameters
    /// to be serializable as JSON and attaches custom headers to the request.
    ///
    /// # Type Parameters
    /// * `T`: A type that implements the [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) trait, representing
    ///   the data to be serialized into JSON and included in the request body.
    ///
    /// # Parameters
    /// * `path`: A [`String`](https://doc.rust-lang.org/std/string/struct.String.html) specifying the REST API endpoint's relative path.
    /// * `params`: A serializable object of type `T` that represents the payload to be included in the PUT request.
    ///
    /// # Returns
    /// * `Result<Response, Error>`: On success, this function returns a successful [`Response`](https://docs.rs/reqwest/latest/reqwest/struct.Response.html) from the PUT request.
    ///   On failure, it returns an [`Error`](https://docs.rs/reqwest/latest/reqwest/enum.Error.html) or custom `Error` defined by the client.
    ///
    /// # Errors
    /// * Returns `Error::NotLoggedIn` if the client is not logged in and the `instance_url` is not available.
    /// * Propagates errors encountered while refreshing the session via `self.ensure_refresh()`.
    /// * Propagates errors from invalid header creation, serializing the payload, sending the HTTP request, or other
    ///   networking issues encountered by the HTTP client.
    ///
    /// # Examples
    /// ```rust
    /// use serde::Serialize;
    /// use rustsf::{Client, Error};
    ///
    /// #[derive(Serialize)]
    /// struct UpdateData {
    ///     key: String,
    ///     value: String,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error>  {
    ///     let mut client = Client::new();
    ///
    ///     let data = UpdateData {
    ///         key: "example_key".to_string(),
    ///         value: "example_value".to_string(),
    ///     };
    ///
    ///     match client.rest_put("/api/resource".to_string(), data).await {
    ///         Ok(response) => {
    ///             println!("Request succeeded with status: {}", response.status());
    ///         }
    ///         Err(e) => {
    ///             eprintln!("Request failed: {}", e);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn rest_put<T: Serialize>(
        &mut self,
        path: String,
        params: T,
    ) -> Result<Response, Error> {
        self.ensure_refresh().await?;

        let url = format!(
            "{}{}",
            self.instance_url.as_ref().ok_or(Error::NotLoggedIn)?,
            path
        );
        let res = self
            .http_client
            .put(url.as_str())
            .headers(self.create_header(vec![])?)
            .json(&params)
            .send()
            .await?;
        Ok(res)
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
    ///   - If refreshing the authentication token fails.
    ///   - If there is an issue creating the HTTP headers.
    ///   - If the HTTP request fails during execution.
    ///
    /// # Async Behavior
    /// - This function is asynchronous and should be awaited.
    /// - Before sending the DELETE request, it ensures that the authentication state is up-to-date by calling `self.ensure_refresh()`.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let response = client.rest_delete("/resource/123".to_string()).await;
    ///
    ///     match response {
    ///         Ok(res) => {
    ///             println!("DELETE request successful: {}", res.status());
    ///         }
    ///         Err(err) => {
    ///             eprintln!("Error occurred: {}", err);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn rest_delete(&mut self, path: String) -> Result<Response, Error> {
        self.ensure_refresh().await?;

        let url = format!(
            "{}{}",
            self.instance_url.as_ref().ok_or(Error::NotLoggedIn)?,
            path
        );
        let res = self
            .http_client
            .delete(url.as_str())
            .headers(self.create_header(vec![])?)
            .send()
            .await?;
        Ok(res)
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
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let response = client.get(
    ///         "https://api.example.com/items".into(),
    ///         vec![("key".into(), "value".into())],
    ///         vec![]
    ///     ).await;
    ///     match response {
    ///         Ok(res) => {
    ///             println!("Response: {:?}", res);
    ///         },
    ///         Err(err) => {
    ///             eprintln!("Error: {:?}", err);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(
        &mut self,
        url: String,
        params: Vec<(String, String)>,
        headers: Vec<(String, String)>,
    ) -> Result<Response, Error> {
        self.ensure_refresh().await?;

        debug!("GET: {} params: {:?} headers: {:?}", url, params, headers);

        let res = self
            .http_client
            .get(url.as_str())
            .headers(self.create_header(headers)?)
            .query(&params)
            .send()
            .await?;
        Ok(res)
    }

    /// Sends a GET request to the specified URL with optional additional headers,
    /// ensuring that necessary token or session refresh procedures are completed before the request.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice that holds the URL to which the GET request will be sent.
    /// * `additional_headers` - A vector of key-value pairs (`String`, `String`) representing
    ///   additional headers to include in the request. Note that the "Accept" header is removed
    ///   from the headers before the request is sent.
    ///
    /// # Returns
    ///
    /// * `Result<Response, Error>` - On success, returns a `Response` object representing the
    ///   HTTP response. On failure, returns an `Error` indicating what went wrong.
    ///
    /// # Errors
    ///
    /// This function returns an error in the following scenarios:
    ///
    /// * If `self.ensure_refresh()` fails during the session or token refresh process.
    /// * If `self.create_header()` fails to create the HTTP headers from `additional_headers`.
    /// * If the HTTP client encounters an issue while sending the GET request.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let  url = "https://example.com/api/resource";
    ///     let  headers = vec![("Authorization".to_string(), "Bearer token".to_string())];
    ///     match client.get_raw(url, headers).await {
    ///         Ok(response) => println!("Response status: {}", response.status()),
    ///         Err(err) => println!("Error: {}", err),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// This method removes the "Accept" header (if any) from the list of headers before
    /// sending the request.
    pub async fn get_raw(
        &mut self,
        url: &str,
        additional_headers: Vec<(String, String)>,
    ) -> Result<Response, Error> {
        self.ensure_refresh().await?;

        debug!("GET: {} headers: {:?}", url, additional_headers);

        let mut headers = self.create_header(additional_headers)?;
        headers.remove("Accept");
        let res = self.http_client.get(url).headers(headers).send().await?;
        Ok(res)
    }

    /// Asynchronously sends an HTTP POST request to the specified URL with the given parameters and headers.
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
    /// * `params` - A serializable type (`T`) used as the JSON body of the POST request.
    /// * `headers` - A vector of tuples where each tuple contains a header name and its corresponding value (`Vec<(String, String)>`).
    ///
    /// # Returns
    ///
    /// Returns a `Result`:
    /// - `Ok(Response)` on success, where `Response` is the HTTP response returned by the request.
    /// - `Err(Error)` if an error occurs during the request, such as serialization issues, header creation errors, or HTTP client errors.
    ///
    /// # Errors
    ///
    /// This function may return an error in the following circumstances:
    /// * If `self.ensure_refresh()` fails.
    /// * If `self.create_header(headers)` fails to construct the headers.
    /// * If the HTTP client encounters an issue while sending the request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustsf::{Client, BulkApiV2, Error, DefSObject};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[DefSObject(sobject_type = "Account", fields="system,type,name")]
    /// struct Account {}
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     use rustsf::primary_types::SObject;
    /// let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let headers = vec![("Authorization".to_string(), "Bearer token".to_string())];
    ///     let url = "https://www.salesforce.com/api/v60.0".to_string();
    ///     let mut acc = Account::new();
    ///     acc.set_id(Some("001xx000003DGbX"));
    ///     acc.name = Some("Test".to_string());
    ///
    ///     let response = client.post(url, acc, headers).await;
    ///     match response {
    ///         Ok(res) => println!("Response: {:?}", res),
    ///         Err(err) => eprintln!("Error: {:?}", err),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Note: Ensure that the given `url` is valid and accessible and that proper error handling is implemented
    /// for production use.
    pub async fn post<T: Serialize + Debug>(
        &mut self,
        url: String,
        params: T,
        headers: Vec<(String, String)>,
    ) -> Result<Response, Error> {
        self.ensure_refresh().await?;

        debug!("POST: {} params: {:?} headers: {:?}", url, params, headers);

        let res = self
            .http_client
            .post(url)
            .headers(self.create_header(headers)?)
            .json(&params)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn post_soap(&mut self, action: &str, body: String) -> Result<Response, Error> {

        let url = self.soap_version_path()?;
        debug!("Soap Metadata API '{}' request: POST {} : {}", action, url, body );

        Ok(self.http_client
            .post(url)
            .header("Content-Type", "text/xml")
            .header("SOAPAction", format!("'{}'", action))
            .body(body)
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
    pub async fn post_multipart(&mut self, url: String, headers: Vec<(String, String)>, form: Form) -> Result<Response, Error> {
        println!("url {:?}", url);

        let request = self.http_client
            .post(&url)
            .multipart(form)
            .headers(self.create_header(headers)?)
            .build()?;

        println!("Request URL: {}", request.url());
        println!("Request Method: {}", request.method());

        // Log headers
        for (name, value) in request.headers() {
            println!("Header {}: {:?}", name, value);
        }

        // Log body size (reqwest hides the exact body bytes if it's a stream/json,
        // but you can check if a body exists)
        if let Some(body) = request.body() {
            println!("Body: {:?}", body.as_bytes());
        }

        let response = self.http_client.execute(request).await?;

        Ok(response)
    }


    /// Asynchronously sends an HTTP POST request to the specified URL with the given parameters and headers.
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
    /// * `params` - A serializable type (`T`) used as the JSON body of the POST request.
    /// * `headers` - A vector of tuples where each tuple contains a header name and its corresponding value (`Vec<(String, String)>`).
    ///
    /// # Returns
    ///
    /// Returns a `Result`:
    /// - `Ok(Response)` on success, where `Response` is the HTTP response returned by the request.
    /// - `Err(Error)` if an error occurs during the request, such as serialization issues, header creation errors, or HTTP client errors.
    ///
    /// # Errors
    ///
    /// This function may return an error in the following circumstances:
    /// * If `self.ensure_refresh()` fails.
    /// * If `self.create_header(headers)` fails to construct the headers.
    /// * If the HTTP client encounters an issue while sending the request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let headers = vec![("Authorization".to_string(), "Bearer token".to_string())];
    ///     let url = "https://example.com/api/accounts".to_string();
    ///     match client.post_raw_buffer(url, vec![], headers).await {
    ///         Ok(res) => println!("Response: {:?}", res),
    ///         Err(err) => println!("Error: {:?}", err),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn post_raw_buffer(
        &mut self,
        url: String,
        body: Vec<u8>,
        headers: Vec<(String, String)>,
    ) -> Result<Response, Error> {
        self.ensure_refresh().await?;

        debug!("POST: {} body: {:?} headers: {:?}", url, body, headers);

        let res = self
            .http_client
            .post(url)
            .headers(self.create_header(headers)?)
            .body(body)
            .send()
            .await?;
        Ok(res)
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let url = "https://example.com/resource".to_string();
    ///     let data = b"column1,column2\nvalue1,value2".to_vec();
    ///
    ///     match client.put(url, data).await {
    ///         Ok(response) => println!("PUT request succeeded with status: {}", response.status()),
    ///         Err(err) => eprintln!("PUT request failed: {}", err),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn put(&mut self, url: String, buffer: Vec<u8>) -> Result<Response, Error> {
        self.ensure_refresh().await?;

        debug!("PUT: {} body: {:?}", url, buffer);

        let mut headers = self.create_header(vec![])?;
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

    /// Sends an asynchronous HTTP PATCH request to the specified URL with the provided parameters.
    ///
    /// # Type Parameters
    /// - `T`: A type that implements the `Serialize` trait, representing the body of the request.
    ///
    /// # Parameters
    /// - `url`: A `String` containing the target URL for the PATCH request.
    /// - `params`: A value of type `T`, used as the serialized JSON body of the PATCH request.
    ///
    /// # Returns
    /// - `Result<Response, Error>`:
    ///   - `Ok(Response)`: If the request is successful, returns the response from the server.
    ///   - `Err(Error)`: If an error occurs during the request (e.g., problems in refreshing state,
    ///     creating headers, JSON serialization, or network issues), returns the corresponding error.
    ///
    /// # Errors
    /// This function returns an error if:
    /// - `ensure_refresh` fails to refresh the client state.
    /// - An error occurs while creating the headers.
    /// - An error occurs in serializing the `params`.
    /// - The HTTP client fails to send the request or receives an error response.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, Error, DefSObject};
    /// use serde::Serialize;
    ///
    /// #[DefSObject(sobject_type = "Account", fields="name")]
    /// struct Account { }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let url = String::from("https://example.com/resource");
    ///     let mut acc = Account::new();
    ///     acc.id = Some(String::from("001D000000IqhSLIAZ"));
    ///     acc.name = Some(String::from("Sample Account"));
    ///
    ///     match client.patch::<Account>(url, acc).await {
    ///         Ok(response) => println!("Request succeeded: {:?}", response),
    ///         Err(err) => eprintln!("Request failed: {}", err),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn patch<T: Serialize + Debug>(
        &mut self,
        url: String,
        params: T,
    ) -> Result<Response, Error> {
        self.ensure_refresh().await?;

        debug!("PATCH: {} params: {:?}", url, params);

        let res = self
            .http_client
            .patch(url.as_str())
            .headers(self.create_header(vec![])?)
            .json(&params)
            .send()
            .await?;
        Ok(res)
    }

    ///
    /// Sends an HTTP DELETE request to the specified URL and returns the response.
    ///
    /// This function performs the following operations:
    /// 1. Ensures that any necessary refresh operations are completed by calling `ensure_refresh`.
    /// 2. Constructs a DELETE request to the given URL using the internal HTTP client.
    /// 3. Adds the necessary headers to the request (created via `create_header`).
    /// 4. Sends the request asynchronously and waits for the response.
    ///
    /// # Parameters
    /// - `url`: A `String` representing the URL to which the DELETE request will be sent.
    ///
    /// # Returns
    /// - `Ok(Response)`: An HTTP response if the request is successful.
    /// - `Err(Error)`: An error if any part of the process fails, such as constructing headers,
    ///   sending the request, or encountering issues during the refresh process.
    ///
    /// # Errors
    /// This function will return an `Error` in the following cases:
    /// - If `ensure_refresh` fails.
    /// - If there is an issue with creating the headers.
    /// - If sending the DELETE request fails (e.g., network issues or server errors).
    ///
    /// # Examples
    /// ```
    /// use rustsf::{Client, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let url = "https://api.example.com/resource".to_string();
    ///     match client.delete(url).await {
    ///         Ok(response) => println!("DELETE request succeeded with status: {}", response.status()),
    ///         Err(err) => eprintln!("DELETE request failed: {}", err),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Note
    /// Ensure that the client is properly initialized and authenticated before calling this method.
    ///
    pub async fn delete(&mut self, url: String) -> Result<Response, Error> {
        self.ensure_refresh().await?;

        debug!("DELETE: {}", url);

        let res = self
            .http_client
            .delete(url.as_str())
            .headers(self.create_header(vec![])?)
            .send()
            .await?;
        Ok(res)
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
    fn create_header(&self, additional_headers: Vec<(String, String)>) -> Result<HeaderMap, Error> {
        let mut headers = HeaderMap::new();
        let auth_value = format!(
            "Bearer {}",
            self.access_token.as_ref().ok_or(Error::NotLoggedIn)?.value
        );
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_value)?);

        //Default header
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        for (key, value) in additional_headers {
            let header_name: HeaderName = key
                .parse()
                .map_err(|_| Error::ConfigError(format!("Invalid header name: {}", key)))?;

            let header_value = HeaderValue::from_str(&value)?;

            //delete duplicates
            headers.remove(&header_name);

            headers.insert(header_name, header_value);
        }

        Ok(headers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use serde_json::json;

    fn create_test_client(server_url: &str) -> Client {
        let mut client = Client::new();
        client.set_instance_url(server_url);
        client.set_access_token(
            "test_token".to_string(),
            // Use a timestamp far in the future so ensure_refresh doesn't trigger
            "9999999999000".to_string(),
            "Bearer".to_string(),
        );
        client
    }

    // --- Constructor and defaults ---

    #[test]
    fn test_new_defaults() {
        let client = Client::new();
        assert_eq!(client.login_endpoint, "https://login.salesforce.com");
        assert_eq!(client.version, "v60.0");
        assert!(client.client_id.is_none());
        assert!(client.client_secret.is_none());
        assert!(client.access_token.is_none());
        assert!(client.instance_url.is_none());
        assert!(client.refresh_token.is_none());
        assert!(client.secret_required);
    }

    #[test]
    fn test_default_calls_new() {
        let client = Client::default();
        assert_eq!(client.version, "v60.0");
    }

    // --- Setters and getters ---

    #[test]
    fn test_set_login_endpoint() {
        let mut client = Client::new();
        let result = client.set_login_endpoint("https://test.salesforce.com");
        assert_eq!(result.login_endpoint, "https://test.salesforce.com");
    }

    #[test]
    fn test_set_version() {
        let mut client = Client::new();
        client.set_version("v55.0");
        assert_eq!(client.version, "v55.0");
    }

    #[test]
    fn test_set_instance_url() {
        let mut client = Client::new();
        client.set_instance_url("https://example.com");
        assert_eq!(Some("https://example.com".to_string()), client.instance_url);
    }

    #[test]
    fn test_instance_url_getter() {
        let mut client = Client::new();
        assert_eq!(None, client.instance_url());
        client.set_instance_url("https://example.com");
        assert_eq!(Some("https://example.com"), client.instance_url());
    }

    #[test]
    fn test_set_refresh_token() {
        let mut client = Client::new();
        client.set_refresh_token("my_refresh_token");
        assert_eq!(Some("my_refresh_token".to_string()), client.refresh_token);
    }

    #[test]
    fn test_set_secret_required() {
        let mut client = Client::new();
        assert!(client.secret_required);
        client.set_secret_required(false);
        assert!(!client.secret_required);
    }

    #[test]
    fn test_set_client_id() {
        let mut client = Client::new();
        client.set_client_id("my_client_id");
        assert_eq!(Some("my_client_id".to_string()), client.client_id);
    }

    #[test]
    fn test_set_client_secret() {
        let mut client = Client::new();
        client.set_client_secret("my_secret");
        assert_eq!(Some("my_secret".to_string()), client.client_secret);
    }

    #[test]
    fn test_set_access_token() {
        let mut client = Client::new();
        client.set_access_token(
            "token_val".to_string(),
            "issued".to_string(),
            "Bearer".to_string(),
        );
        let token = client.access_token.as_ref().unwrap();
        assert_eq!("token_val", token.value);
        assert_eq!("issued", token.issued_at);
        assert_eq!("Bearer", token.token_type);
    }

    #[test]
    fn test_access_token_value() {
        let mut client = Client::new();
        assert_eq!(None, client.access_token_value());
        client.set_access_token("abc".to_string(), "".to_string(), "".to_string());
        assert_eq!(Some("abc"), client.access_token_value());
    }

    #[test]
    fn test_read_only_getters() {
        let mut client = Client::new();
        client.set_client_id("cid");
        client.set_client_secret("csecret");
        client.set_refresh_token("rtoken");

        assert_eq!(client.client_id(), Some("cid"));
        assert_eq!(client.client_secret(), Some("csecret"));
        assert_eq!(client.login_endpoint(), "https://login.salesforce.com");
        assert_eq!(client.version(), "v60.0");
        assert_eq!(client.refresh_token(), Some("rtoken"));
    }

    // --- Chaining setters ---

    #[test]
    fn test_setter_chaining() {
        let mut client = Client::new();
        client
            .set_login_endpoint("https://test.salesforce.com")
            .set_version("v55.0")
            .set_instance_url("https://inst.salesforce.com")
            .set_client_id("cid")
            .set_client_secret("csecret")
            .set_refresh_token("rtoken")
            .set_secret_required(false);

        assert_eq!(client.login_endpoint, "https://test.salesforce.com");
        assert_eq!(client.version, "v55.0");
        assert_eq!(
            client.instance_url,
            Some("https://inst.salesforce.com".to_string())
        );
        assert_eq!(client.client_id, Some("cid".to_string()));
        assert_eq!(client.client_secret, Some("csecret".to_string()));
        assert_eq!(client.refresh_token, Some("rtoken".to_string()));
        assert!(!client.secret_required);
    }

    // --- Refresh params ---

    #[test]
    fn test_get_refresh_params_with_secret() {
        let mut client = Client::new();
        client.set_client_id("cid");
        client.set_client_secret("csecret");
        client.set_refresh_token("rtoken");

        let params = client.get_refresh_params();
        assert_eq!(params.len(), 4);
        assert!(params.contains(&("grant_type".to_string(), "refresh_token".to_string())));
        assert!(params.contains(&("refresh_token".to_string(), "rtoken".to_string())));
        assert!(params.contains(&("client_id".to_string(), "cid".to_string())));
        assert!(params.contains(&("client_secret".to_string(), "csecret".to_string())));
    }

    #[test]
    fn test_get_refresh_params_without_secret() {
        let mut client = Client::new();
        client.set_secret_required(false);
        client.set_client_id("cid");
        client.set_refresh_token("rtoken");

        let params = client.get_refresh_params();
        assert_eq!(params.len(), 3);
        assert!(!params.iter().any(|(k, _)| k == "client_secret"));
    }

    #[test]
    fn test_get_refresh_params_defaults_when_none() {
        let client = Client::new();
        let params = client.get_refresh_params();
        // Should use empty strings for missing values
        assert!(params.contains(&("refresh_token".to_string(), "".to_string())));
        assert!(params.contains(&("client_id".to_string(), "".to_string())));
    }

    // --- create_header ---

    #[test]
    fn test_create_header_not_logged_in() {
        let client = Client::new();
        let result = client.create_header(vec![]);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::NotLoggedIn => {}
            e => panic!("Expected NotLoggedIn, got {:?}", e),
        }
    }

    #[test]
    fn test_create_header_with_token() {
        let mut client = Client::new();
        client.set_access_token("mytoken".to_string(), "".to_string(), "Bearer".to_string());

        let headers = client.create_header(vec![]).unwrap();
        assert_eq!(headers.get("Authorization").unwrap(), "Bearer mytoken");
        assert_eq!(headers.get("Accept").unwrap(), "application/json");
    }

    #[test]
    fn test_create_header_with_additional_headers() {
        let mut client = Client::new();
        client.set_access_token("mytoken".to_string(), "".to_string(), "Bearer".to_string());

        let headers = client
            .create_header(vec![
                ("X-Custom".to_string(), "custom_value".to_string()),
                ("Accept".to_string(), "text/xml".to_string()),
            ])
            .unwrap();

        assert_eq!(headers.get("X-Custom").unwrap(), "custom_value");
        // Accept should be overridden
        assert_eq!(headers.get("Accept").unwrap(), "text/xml");
    }

    // --- ensure_refresh ---

    #[tokio::test]
    async fn test_ensure_refresh_no_token() {
        let mut client = Client::new();
        // Should return Ok without doing anything
        let result = client.ensure_refresh().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_ensure_refresh_token_not_expired() {
        let mut client = Client::new();
        // Set a token with issued_at far in the future
        client.set_access_token(
            "token".to_string(),
            "9999999999000".to_string(),
            "Bearer".to_string(),
        );
        let result = client.ensure_refresh().await;
        assert!(result.is_ok());
        assert_eq!(client.access_token_value(), Some("token"));
    }

    // --- login_with_credential ---

    #[tokio::test]
    async fn test_login_with_credential_success() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/services/oauth2/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "access_token": "PowerLevel9000",
                    "issued_at": "1234567890000",
                    "id": "https://login.salesforce.com/id/00Dxx/005xx",
                    "instance_url": server.url(),
                    "signature": "sig",
                    "token_type": "Bearer",
                })
                    .to_string(),
            )
            .create_async()
            .await;

        let mut client = Client::new();
        client.set_client_id("cid");
        client.set_client_secret("csecret");
        client.set_login_endpoint(&server.url());

        let result = client.login_with_credential("user", "pass").await;

        assert!(result.is_ok());
        assert_eq!(client.access_token_value(), Some("PowerLevel9000"));
        assert_eq!(client.instance_url.unwrap(), server.url());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_login_with_credential_failure() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/services/oauth2/token")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "error": "invalid_grant",
                    "error_description": "authentication failure"
                })
                    .to_string(),
            )
            .create_async()
            .await;

        let mut client = Client::new();
        client.set_client_id("cid");
        client.set_client_secret("csecret");
        client.set_login_endpoint(&server.url());

        let result = client.login_with_credential("user", "pass").await;

        assert!(result.is_err());
        match result.unwrap_err() {
            Error::TokenError(_) => {}
            e => panic!("Expected TokenError, got {:?}", e),
        }
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_login_with_credential_missing_client_id() {
        let mut client = Client::new();
        let result = client.login_with_credential("user", "pass").await;
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::ConfigError(msg) => assert!(msg.contains("client_id")),
            e => panic!("Expected ConfigError, got {:?}", e),
        }
    }

    // --- refresh ---

    #[tokio::test]
    async fn test_refresh_success() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/services/oauth2/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "access_token": "new_token",
                    "issued_at": "1234567890000",
                    "id": "https://login.salesforce.com/id/00Dxx/005xx",
                    "instance_url": server.url(),
                    "signature": "sig",
                    "token_type": "Bearer",
                })
                    .to_string(),
            )
            .create_async()
            .await;

        let mut client = Client::new();
        client.set_client_id("cid");
        client.set_client_secret("csecret");
        client.set_refresh_token("rtoken");
        client.set_login_endpoint(&server.url());

        let result = client.refresh().await;
        assert!(result.is_ok());
        assert_eq!(client.access_token_value(), Some("new_token"));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_refresh_failure() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/services/oauth2/token")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "error": "invalid_grant",
                    "error_description": "expired refresh token"
                })
                    .to_string(),
            )
            .create_async()
            .await;

        let mut client = Client::new();
        client.set_login_endpoint(&server.url());

        let result = client.refresh().await;
        assert!(result.is_err());
        mock.assert_async().await;
    }

    // --- login_by_soap ---

    #[tokio::test]
    async fn test_login_by_soap_success() {
        let mut server = Server::new_async().await;
        let soap_response = r#"<?xml version="1.0" encoding="UTF-8"?>
            <soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/">
                <soapenv:Body>
                    <loginResponse>
                        <result>
                            <sessionId>soap_token_123</sessionId>
                            <serverUrl>https://na1.salesforce.com/services/Soap/u/60.0/00Dxx</serverUrl>
                            <serverTimestamp>2024-01-01T00:00:00.000Z</serverTimestamp>
                        </result>
                    </loginResponse>
                </soapenv:Body>
            </soapenv:Envelope>"#;

        let mock = server
            .mock("POST", "/services/Soap/u/v60.0")
            .with_status(200)
            .with_header("content-type", "text/xml")
            .with_body(soap_response)
            .create_async()
            .await;

        let mut client = Client::new();
        client.set_login_endpoint(&server.url());

        #[allow(deprecated)]
        let result = client.login_with_soap("user", "pass").await;

        assert!(result.is_ok());
        let token = client.access_token.unwrap();
        assert_eq!(token.value, "soap_token_123");
        assert_eq!(token.token_type, "Bearer");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_login_by_soap_failure() {
        let mut server = Server::new_async().await;
        let soap_error = r#"<?xml version="1.0" encoding="UTF-8"?>
            <soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/">
                <soapenv:Body>
                    <soapenv:Fault>
                        <faultcode>INVALID_LOGIN</faultcode>
                        <faultstring>Invalid username or password</faultstring>
                    </soapenv:Fault>
                </soapenv:Body>
            </soapenv:Envelope>"#;

        let mock = server
            .mock("POST", "/services/Soap/u/v60.0")
            .with_status(500)
            .with_header("content-type", "text/xml")
            .with_body(soap_error)
            .create_async()
            .await;

        let mut client = Client::new();
        client.set_login_endpoint(&server.url());

        #[allow(deprecated)]
        let result = client.login_with_soap("user", "pass").await;

        assert!(result.is_err());
        match result.unwrap_err() {
            Error::LoginError(e) => {
                assert_eq!(e.error_code, "INVALID_LOGIN");
                assert_eq!(e.message, "Invalid username or password");
            }
            e => panic!("Expected LoginError, got {:?}", e),
        }
        mock.assert_async().await;
    }

    // --- HTTP methods with mock server ---

    #[tokio::test]
    async fn test_get() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .match_query(mockito::Matcher::AllOf(vec![]))
            .with_status(200)
            .with_body("ok")
            .create_async()
            .await;

        let mut client = create_test_client(&server.url());
        let res = client
            .get(format!("{}/test", server.url()), vec![], vec![])
            .await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap().status(), 200);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_post() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/test")
            .with_status(201)
            .with_body(r#"{"id":"123","success":true}"#)
            .create_async()
            .await;

        let mut client = create_test_client(&server.url());
        let mut params = std::collections::HashMap::new();
        params.insert("Name", "Test");
        let res = client
            .post(format!("{}/test", server.url()), params, vec![])
            .await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap().status(), 201);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_patch() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PATCH", "/test")
            .with_status(204)
            .create_async()
            .await;

        let mut client = create_test_client(&server.url());
        let mut params = std::collections::HashMap::new();
        params.insert("Name", "Updated");
        let res = client.patch(format!("{}/test", server.url()), params).await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap().status(), 204);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_delete() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/test")
            .with_status(204)
            .create_async()
            .await;

        let mut client = create_test_client(&server.url());
        let res = client.delete(format!("{}/test", server.url())).await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap().status(), 204);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_put() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/test")
            .with_status(201)
            .create_async()
            .await;

        let mut client = create_test_client(&server.url());
        let res = client
            .put(format!("{}/test", server.url()), b"csv,data".to_vec())
            .await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap().status(), 201);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_post_raw_buffer() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/test")
            .with_status(200)
            .create_async()
            .await;

        let mut client = create_test_client(&server.url());
        let res = client
            .post_raw_buffer(
                format!("{}/test", server.url()),
                b"raw data".to_vec(),
                vec![("Content-Type".to_string(), "text/csv".to_string())],
            )
            .await;
        assert!(res.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_raw() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(200)
            .with_body("raw response")
            .create_async()
            .await;

        let mut client = create_test_client(&server.url());
        let res = client
            .get_raw(&format!("{}/test", server.url()), vec![])
            .await;
        assert!(res.is_ok());
        mock.assert_async().await;
    }

    // --- REST methods ---

    #[tokio::test]
    async fn test_rest_get() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/some/path")
            .with_status(200)
            .with_body("ok")
            .create_async()
            .await;

        let mut client = create_test_client(&server.url());
        let res = client.rest_get("/some/path".to_string(), vec![]).await;
        assert!(res.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_rest_post() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/some/path")
            .with_status(201)
            .create_async()
            .await;

        let mut client = create_test_client(&server.url());
        let mut params = std::collections::HashMap::new();
        params.insert("key", "value");
        let res = client.rest_post("/some/path".to_string(), params).await;
        assert!(res.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_rest_patch() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PATCH", "/some/path")
            .with_status(204)
            .create_async()
            .await;

        let mut client = create_test_client(&server.url());
        let mut params = std::collections::HashMap::new();
        params.insert("key", "value");
        let res = client.rest_patch("/some/path".to_string(), params).await;
        assert!(res.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_rest_put() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/some/path")
            .with_status(200)
            .create_async()
            .await;

        let mut client = create_test_client(&server.url());
        let mut params = std::collections::HashMap::new();
        params.insert("key", "value");
        let res = client.rest_put("/some/path".to_string(), params).await;
        assert!(res.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_rest_delete() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/some/path")
            .with_status(204)
            .create_async()
            .await;

        let mut client = create_test_client(&server.url());
        let res = client.rest_delete("/some/path".to_string()).await;
        assert!(res.is_ok());
        mock.assert_async().await;
    }

    // --- get_identity ---

    #[tokio::test]
    async fn test_get_identity_success() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/id/info")
            .with_status(200)
            .with_body(r#"{"user_id":"005xx","username":"test@test.com"}"#)
            .create_async()
            .await;

        let mut client = create_test_client(&server.url());
        let result = client
            .get_identity(format!("{}/id/info", server.url()))
            .await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("test@test.com"));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_identity_failure() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/id/info")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "message": "Session expired",
                    "errorCode": "INVALID_SESSION_ID"
                })
                    .to_string(),
            )
            .create_async()
            .await;

        let mut client = create_test_client(&server.url());
        let result = client
            .get_identity(format!("{}/id/info", server.url()))
            .await;
        assert!(result.is_err());
        mock.assert_async().await;
    }

    // --- rest_get_fulluri ---

    #[tokio::test]
    async fn test_rest_get_fulluri() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/services/apexrest/MyEndpoint")
            .match_query(mockito::Matcher::UrlEncoded("param".into(), "value".into()))
            .with_status(200)
            .with_body("ok")
            .create_async()
            .await;

        let mut client = create_test_client(&server.url());
        let res = client.rest_get_fulluri("MyEndpoint?param=value").await;
        assert!(res.is_ok());
        mock.assert_async().await;
    }

    // --- ensure_refresh with expired token ---

    #[tokio::test]
    async fn test_ensure_refresh_expired_token_triggers_refresh() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/services/oauth2/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "access_token": "refreshed_token",
                    "issued_at": "9999999999000",
                    "id": "https://login.salesforce.com/id/00Dxx/005xx",
                    "instance_url": server.url(),
                    "signature": "sig",
                    "token_type": "Bearer",
                })
                    .to_string(),
            )
            .create_async()
            .await;

        let mut client = Client::new();
        client.set_login_endpoint(&server.url());
        client.set_client_id("cid");
        client.set_client_secret("csecret");
        client.set_refresh_token("rtoken");
        // Set a token with issued_at far in the past (expired > 2 hours ago)
        client.set_access_token(
            "old_token".to_string(),
            "1000000000000".to_string(), // ~2001, well past 2 hours
            "Bearer".to_string(),
        );

        let result = client.ensure_refresh().await;
        assert!(result.is_ok());
        assert_eq!(client.access_token_value(), Some("refreshed_token"));
        mock.assert_async().await;
    }
}