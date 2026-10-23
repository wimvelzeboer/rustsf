use std::fmt::Debug;

use serde::Serialize;

use crate::RestApi;
use crate::rest_api::HTTPMethod;
use crate::rest_api::builder::sobjects::SObjectsBuilder;
use crate::rest_api::error::RestApiError;

impl RestApi {
	///
	/// Updates an existing object in a Salesforce instance with the specified parameters.
	///
	/// # Generic Parameters
	/// - `T`: A type that implements the `Serialize` trait, representing the parameters to update the object with.
	///
	/// # Arguments
	/// - `object_name`: A string slice that specifies the name of the Salesforce object (e.g., "Account", "Contact").
	/// - `id`: A string slice representing the unique ID of the object to be updated.
	/// - `params`: Parameters of type `T` containing the fields and values to update on the object.
	///
	/// # Returns
	/// - `Ok(())` if the object was successfully updated.
	/// - `Err(Error)` if an error occurs during the update process, including issues with the request or response.
	///
	/// # Errors
	/// This function returns an error if:
	/// - The provided `object_name`, `id`, or `params` result in an invalid or malformed request.
	/// - The Salesforce API response indicates a failure, such as an invalid ID or insufficient permissions.
	/// - The internal HTTP client encounters an error while making the `PATCH` request.
	///
	/// # Example
	/// ```
	/// use rustsf::{Client, Credentials, RestApi, DefSObject};
	/// use serde::{Deserialize, Serialize};
	/// use anyhow::Result;
	///
	/// #[DefSObject(sobject_type = "Account", fields="system,type,name")]
	/// struct Account { }
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let mut client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///
	///     let mut api = RestApi::new(client);
	///     let mut account = Account::new();
	///     account.id = Some("001D000000IqhSLIAZ".to_string());
	///     account.name = Some("Updated Account Name".to_string());
	///
	///     match api.update_sobject("Account", "001D000000IqhSLIAZ", account).await {
	///         Ok(()) => println!("Account updated successfully."),
	///         Err(error) => println!("Error updating account: {:?}", error),
	///     };
	///     Ok(())
	/// }
	/// ```
	///
	/// # Notes
	/// - This function makes an asynchronous HTTP `PATCH` request to the Salesforce API.
	/// - It processes the response to ensure that no content (204 No Content) implies a successful update.
	///
	/// # See
	/// https://developer.salesforce.com/docs/platform/api-rest/guide/resources-sobject-retrieve-patch.html
	pub async fn update_sobject<T: Debug + Serialize>(
		&self,
		sobject_name: &str,
		id: &str,
		data: T,
	) -> Result<(), RestApiError> { // todo Return a response here?
		self.build()
			.sobjects()
			.update(sobject_name, id, data)
			.send()
			.await
	}
}

impl<'a> SObjectsBuilder<'a> {
	pub fn update<T: Serialize + Debug>(
		&self,
		object_name: &str,
		id: &str,
		data: T,
	) -> SObjectUpdateBuilder<'_, T> {
		SObjectUpdateBuilder::new(self, object_name, id, data)
	}
}

pub struct SObjectUpdateBuilder<'a, T: Serialize + Debug> {
	builder: &'a SObjectsBuilder<'a>,
	sobject_name: String,
	id: String,
	data: T,
}

impl<'a, T: Serialize + Debug> SObjectUpdateBuilder<'a, T> {
	pub(crate) fn new(
		builder: &'a SObjectsBuilder<'a>,
		sobject_type: &str,
		id: &str,
		data: T,
	) -> SObjectUpdateBuilder<'a, T>
	where
		T: Serialize + Debug,
	{
		Self {
			builder,
			sobject_name: sobject_type.to_string(),
			id: id.to_string(),
			data,
		}
	}

	pub async fn send(&self) -> Result<(), RestApiError> {
		let url = format!("{}/{}", self.sobject_name, self.id);
		let response = self.builder.send(HTTPMethod::PATCH, url, &self.data).await?;
		RestApi::handle_empty_response(response).await
	}
}
