//! # User Password API
//!
//! Module containing all the User Password API operations.
//!
//! ## Supported Endpoints
//! - **/services/data/vXX.X/sobjects/User/password**
//!
//! ## Methods
//! - [user_password_expired](crate::rest_api::RestApi#method.user_password_expired), Checks if a password has expired.
//! - [user_password_reset](crate::rest_api::RestApi#method.user_password_reset), Resets a password.
//! - [user_password_set](crate::rest_api::RestApi#method.user_password_reset), Sets the password for a user.
//!
use std::collections::HashMap;
use crate::rest_api::{handle_json_response, RestApi};
use anyhow::{anyhow, Result};

impl RestApi {

    /// Checks if a user's password has expired in the system.
    ///
    /// # Arguments
    /// * `user_id` - A string slice that represents the ID of the user whose password expiration status
    ///               is to be checked.
    ///
    /// # Returns
    /// * `Result<bool, Error>` - An `Ok` variant containing `true` if the user's password is expired,
    ///                           `false` otherwise. Returns an `Err` variant if there is an error
    ///                           during the request or if the response is invalid.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, RestApi};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let mut api = RestApi::new(client);
    ///     match api.user_password_expired("user_id").await {
    ///         Ok(is_expired) => println!("The new password expired?: {}", is_expired),
    ///         Err(e) => println!("Failed to reset password: {}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/dome_sobject_user_password.htm>
    pub async fn user_password_expired(&mut self, user_id: &str) -> Result<bool> {
        let url = format!(
            "{}/sobjects/User/{}/password",
            self.client.base_version_path()?,
            user_id
        );
        let response = self.client.get(url, vec![], vec![]).await?;
        let response: HashMap<String, bool> = handle_json_response(response).await?;
        match response.get("isExpired") {
            Some(is_expired) => Ok(*is_expired),
            None => Err(anyhow!("Expected 'isExpired' property".to_string())),
        }
    }

    /// Resets the password for a user specified by `user_id`.
    ///
    /// This asynchronous function sends a DELETE request to the appropriate endpoint
    /// to reset the user's password. Upon success, it retrieves and returns the new
    /// password associated with the user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - A string slice that holds the identifier of the user whose password
    ///   needs to be reset.
    ///
    /// # Returns
    ///
    /// This function returns a `Result`:
    /// - On success, an `Ok` variant containing the new password as a `String`.
    /// - On failure, an `Err` variant containing an `Error` describing the cause of failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, RestApi};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let mut api = RestApi::new(client);
    ///     match api.user_password_reset("user_id").await {
    ///         Ok(new_password) => println!("The new password is: {}", new_password),
    ///         Err(e) => println!("Failed to reset password: {}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/dome_sobject_user_password.htm>
    pub async fn user_password_reset(&mut self, user_id: &str)-> Result<String> {
        let url = format!(
            "{}/sobjects/User/{}/password",
            self.client.base_version_path()?,
            user_id
        );
        let response = self.client.delete(url).await?;
        let response: HashMap<String, String> = handle_json_response(response).await?;
        match response.get("NewPassword") {
            Some(new_password) => Ok(new_password.to_string()),
            None => Err(anyhow!("Expected 'NewPassword' property".to_string()))
        }
    }

    /// Sets a new password for a user in the system asynchronously.
    ///
    /// # Arguments
    ///
    /// * `user_id` - A string slice representing the unique identifier of the user
    ///   whose password needs to be updated.
    /// * `password` - A string slice representing the new password to be set for the user.
    ///
    /// # Returns
    ///
    /// * `Result<bool, Error>` - On success, returns `Ok(true)` if the operation was successful
    ///   or `Ok(false)` if the `isExpired` flag indicates the password is expired. On failure,
    ///   returns an `Error`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, RestApi};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let mut api = RestApi::new(client);
    ///     match api.user_password_set("user_id", "password").await {
    ///         Ok(new_password) => println!("Successful password change"),
    ///         Err(e) => println!("Failed to reset password: {:?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// The function constructs the URL endpoint dynamically using the base version path and
    /// appends the user ID. It sends a `POST` request with the `NewPassword` parameter to update
    /// the password. The `isExpired` flag in the JSON response determines whether the new
    /// password is considered expired.
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/dome_sobject_user_password.htm>
    pub async fn user_password_set(&mut self, user_id: &str, password: &str) -> Result<()> {
        let url = format!(
            "{}/sobjects/User/{}/password",
            self.client.base_version_path()?,
            user_id
        );
        let params = vec![("NewPassword", password)];
        self.client.post(url, params, vec![]).await?;
        Ok(())
    }
}