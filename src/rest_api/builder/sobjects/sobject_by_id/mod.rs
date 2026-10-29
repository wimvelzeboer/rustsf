use serde::de::DeserializeOwned;
use crate::rest_api::builder::sobjects::SObjectsBuilder;
use crate::errors::Error;
use crate::rest_api::HTTPMethod;
use crate::rest_api::responses::rest_response::RestResponse;
use crate::RestApi;

impl RestApi {
	/// Finds and retrieves a record of the specified Salesforce object type by its ID,
	/// containing all of its field values.
	///
	/// This asynchronous function builds a resource URL using the provided Salesforce object name and
	/// record ID, sends a GET request to Salesforce, and deserializes the JSON response into the
	/// appropriate type.
	///
	/// # Type Parameters
	/// * `T`: The type into which the returned JSON response will be deserialized.
	///   Must implement the `DeserializeOwned` trait.
	///
	/// # Parameters
	/// * `sobject_name`: A string slice representing the name of the Salesforce object (e.g., "Account", "Contact").
	/// * `id`: A string slice representing the unique ID of the Salesforce object record to retrieve.
	///
	/// # Returns
	/// * On success: `Result<T, Error>` containing the deserialized response as type `T`.
	/// * On failure: `Result<T, Error>` containing an error if the request fails or if deserialization fails.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{RestApi, SalesforceCredentials};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let credentials = SalesforceCredentials::sfdx_url("force://PlatformCLI::token@example.com").unwrap();
	///     let api = RestApi::new(credentials).await.unwrap();
	///
	///     // Using a custom struct or serde_json::Value
	///     match api.sobject_by_id::<serde_json::Value>("Account", "001D000000IqhSLIAZ").await {
	///         Ok(record) => println!("Account Data: {:?}", record),
	///         Err(error) => println!("Error retrieving account: {:?}", error),
	///     };
	/// }
	/// ```
	///
	/// # Dependencies
	/// This function depends on the client's `base_path()` method to obtain the base URL and the
	/// `get` method to perform the HTTP GET request. The response is then handled by the
	/// `handle_json_response` utility.
	///
	/// # See
	/// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_retrieve_get.htm>
	pub async fn sobject_by_id<'a, T: DeserializeOwned>(
		&self,
		sobject_name: &str,
		id: &str,
	) -> Result<T, Error> {
       self.build()
           .sobjects()
           .get_by_id(sobject_name, id)
	       .send()
	       .await?
	       .rest_response()
	       .await
	}
}


impl<'a> SObjectsBuilder<'a> {

   /// Retrieves a specific Salesforce sObject by its ID asynchronously.
   ///
   /// # Type Parameters
   /// - `T`: The type into which the response JSON will be deserialized.
   ///   This type must implement the `Deserialize` trait.
   ///
   /// # Arguments
   /// - `sobject_name`: A string slice representing the name of the Salesforce sObject (e.g., "Account", "Contact").
   /// - `id`: A string slice representing the ID of the specific sObject to retrieve.
   ///
   /// # Returns
   /// - `Result<T>`: Returns the deserialized sObject of type `T` on success, or an error if the request fails.
   ///
   /// # Errors
   /// - This function returns an error if the HTTP request fails, the response cannot be deserialized,
   ///   or the Salesforce API returns an error status.
   ///
   /// # Headers
   /// This call can be combined with the following headers:
   /// - [If-Match](crate::rest_api::builder::RestApiBuilder::if_match_header)
   /// - [If-None-Match](crate::rest_api::builder::RestApiBuilder::if_none_match_header)
   /// - [If-Modified-Since](crate::rest_api::builder::RestApiBuilder::if_modified_since_header)
   /// - [If-Unmodified-Since](crate::rest_api::builder::RestApiBuilder::if_unmodified_since_header)
   ///
   /// Response header
   /// - ETag
   ///
   /// # Example
   /// ```rust
   /// use rustsf::{RestApi, SalesforceCredentials};
   /// use serde_json::Value;
   ///
   /// #[tokio::main]
   /// async fn main() {
   ///     let credentials = SalesforceCredentials::sfdx_url("force://PlatformCLI::token@example.com").unwrap();
   ///     let api = RestApi::new(credentials).await.unwrap();
   ///     match api.sobject_by_id::<Value>("Account", "001D000000IqhSLIAZ").await {
   ///         Ok(response) => println!("Record: {:?}", response),
   ///         Err(error) => println!("Error creating account: {:?}", error),
   ///     }
   /// }
   /// ```
   ///
   /// This example retrieves an `Account` object with the given ID and deserializes it into the `Account` struct.
   ///
   /// # Dependencies
   /// - This function internally uses `self.builder.send` to send an HTTP GET request.
   /// - The response is processed using `crate::client::handle_json_response`.
   ///
   /// # See
   /// https://developer.salesforce.com/docs/platform/api-rest/guide/resources-sobject-retrieve-get.html
   pub fn get_by_id(
      &self,
      sobject_type: &str,
      id: &str,
   ) -> SObjectSelectByIdBuilder<'_> {
	   SObjectSelectByIdBuilder::new(self, sobject_type, id)
   }
}


pub struct SObjectSelectByIdBuilder<'a> {
	builder: &'a SObjectsBuilder<'a>,
	sobject_type: String,
	id: String,
}


impl<'a> SObjectSelectByIdBuilder<'a> {
	pub(crate) fn new(builder: &'a SObjectsBuilder, sobject_type: &str, id: &str) -> SObjectSelectByIdBuilder<'a>
	{
		Self {
			builder,
			sobject_type: sobject_type.to_string(),
			id: id.to_string(),
		}
	}

	pub async fn send(
		&self,
	) -> Result<RestResponse, Error> {
		let param: Vec<(&str, &str)> = vec![];
		let response = self.builder.send(
			HTTPMethod::GET,
			format!("{}/{}", self.sobject_type, self.id),
			&param,
		).await?;

		Ok(RestResponse::new(response))
	}
}