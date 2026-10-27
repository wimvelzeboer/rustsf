use crate::rest_api::builder::sobjects::SObjectsBuilder;
use crate::errors::Error;
use crate::RestApi;

impl RestApi {

	/// Deletes a specific record of a given Salesforce object type (`sobject_name`) using its ID.
	///
	/// This asynchronous function constructs the resource URL for the given object type (`sobject_name`)
	/// and record ID (`id`), sends a `DELETE` request to the Salesforce API, and processes the response.
	///
	/// # Arguments
	///
	/// * `sobject_name` - The API name of the Salesforce object. For example, `"Account"`, `"Contact"`, etc.
	/// * `id` - The unique ID of the record to delete.
	///
	/// # Returns
	///
	/// * `Result<(), Error>` - Returns `Ok(())` if the record is successfully deleted.
	///   Returns an `Err(Error)` if there is an issue during the deletion process (e.g., network error, API failure).
	///
	/// # Errors
	///
	/// This function will return an error in the following cases:
	/// - If constructing the resource URL fails.
	/// - If the `DELETE` request to the Salesforce API fails.
	/// - If the response from the API indicates a failure or cannot be processed.
	///
	/// # Examples
	///
	/// ```rust
	/// use rustsf::{RestApi, AuthUrl};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let api = RestApi::new(AuthUrl::new("force://PlatformCLI::token@example.com")?).await?;
	///
	///     let result = api.delete_sobject("Account", "001D000000IqhSLIAZ").await;
	///
	///     match result {
	///         Ok(()) => println!("Record deleted successfully."),
	///         Err(e) => eprintln!("Failed to delete record: {:?}", e),
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # Async
	///
	/// This function is `async` and should be awaited.
	///
	/// # Notes
	///
	/// Ensure that the authenticated Salesforce client (`self.client`) has the necessary permissions
	/// to perform delete operations in the Salesforce org.
	///
	/// # See
	/// https://developer.salesforce.com/docs/platform/api-rest/guide/resources-sobject-retrieve-delete.html
	pub async fn delete_sobject(& self, sobject_type: &str, id: &str) -> Result<(), Error> {
		self.build().sobjects().delete(sobject_type, id).send().await
	}
}

impl<'a> SObjectsBuilder<'a> {

	/// Deletes a specific record of a given Salesforce object type (`sobject_name`) using its ID.
	///
	/// This asynchronous function constructs the resource URL for the given object type (`sobject_name`)
	/// and record ID (`id`), sends a `DELETE` request to the Salesforce API, and processes the response.
	///
	/// # Arguments
	///
	/// * `sobject_name` - The API name of the Salesforce object. For example, `"Account"`, `"Contact"`, etc.
	/// * `id` - The unique ID of the record to delete.
	///
	/// # Returns
	///
	/// * `Result<(), Error>` - Returns `Ok(())` if the record is successfully deleted.
	///   Returns an `Err(Error)` if there is an issue during the deletion process (e.g., network error, API failure).
	///
	/// # Errors
	///
	/// This function will return an error in the following cases:
	/// - If constructing the resource URL fails.
	/// - If the `DELETE` request to the Salesforce API fails.
	/// - If the response from the API indicates a failure or cannot be processed.
	///
	/// # Examples
	///
	/// ```rust
	/// use rustsf::{RestApi, AuthUrl};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let client = RestApi::new(AuthUrl::new("force://PlatformCLI::token@example.com")?).await?;
	///
	///     let result = client.build().sobjects().delete("Account", "001D000000IqhSLIAZ").send().await;
	///
	///     match result {
	///         Ok(()) => println!("Record deleted successfully."),
	///         Err(e) => eprintln!("Failed to delete record: {:?}", e),
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # Async
	///
	/// This function is `async` and should be awaited.
	///
	/// # Notes
	///
	/// Ensure that the authenticated Salesforce client (`self.client`) has the necessary permissions
	/// to perform delete operations in the Salesforce org.
	///
	/// # See
	/// https://developer.salesforce.com/docs/platform/api-rest/guide/resources-sobject-retrieve-delete.html
	pub fn delete(&'a self, sobject_type: &str, id: &str) -> SObjectDeleteBuilder<'a> {
		SObjectDeleteBuilder::new(&self, sobject_type.to_string(), id.to_string())
	}
}

pub struct SObjectDeleteBuilder<'a> {
	builder: &'a SObjectsBuilder<'a>,
	id: String,
	sobject_type: String,
}

impl<'a> SObjectDeleteBuilder<'a> {
	pub(crate) fn new(builder: &'a SObjectsBuilder, sobject_type: String, id: String) -> SObjectDeleteBuilder<'a>
	{
		Self {
			builder,
			id,
			sobject_type,
		}
	}

	/// Sends the delete request to the Salesforce REST API
	///
	/// This asynchronous method sends an HTTP POST request to delete a Salesforce
	/// sObject.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{RestApi, AuthUrl};
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let client = RestApi::new(AuthUrl::new("force://PlatformCLI::token@example.com").unwrap()).await.unwrap();
	///
	///     let account_id = "0012F00002h5G8V";
	///     match client.build()
	/// 			.sobjects()
	/// 			.delete("Account", account_id)
	/// 			.send()
	/// 			.await {
	///         Ok(_) => println!("Record ID {} successfully deleted", account_id),
	///         Err(error) => println!("Error deleting record with Id {}: {:?}", account_id, error),
	///     }
	/// }
	/// ```
	pub async fn send(self) -> Result<(), Error> { // fixme - you want to have a proper error here (non anyhow)
		let http_response = self
			.builder
			.send_delete(format!("{}/{}", self.sobject_type, self.id))
			.await?;

		RestApi::handle_empty_response(http_response).await
	}
}