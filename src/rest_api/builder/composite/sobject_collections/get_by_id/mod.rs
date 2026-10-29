use std::fmt::Debug;
use serde::de::DeserializeOwned;
use crate::rest_api::builder::composite::sobject_collections::SObjectCollectionsBuilder;
use crate::errors::Error;
use crate::rest_api::HTTPMethod;
use crate::RestApi;

impl RestApi {

	/// Fetches records of a specified Salesforce object type by a list of IDs and fields.
	///
	/// This asynchronous function retrieves records of a Salesforce object specified by `object_name`
	/// based on a list of IDs and requested fields. The function sends a composite API request to Salesforce
	/// and parses the response into the specified struct type implementing the `SObject` trait.
	///
	/// # Type Parameters
	/// - `T`: A type representing the Salesforce SObject, which must implement the `Serialize`, `Debug`,
	///   `SObject`, and `Clone` traits.
	///
	/// # Parameters
	/// - `object_name`: The API name of the Salesforce object to fetch records for (e.g., "Account", "Contact").
	/// - `ids`: A vector of Salesforce Ids values, limited to approximately 800 before a HTTP 414 error is returned.
	/// - `fields`: A vector of Salesforce SObjectField names (API names, e.g. "Custom_field__c").
	///
	/// # Returns
	/// - `Result<Vec<T>, Error>`: On success, returns a `Vec<T>` containing the records fetched
	///   from Salesforce that match the provided IDs. Returns an `Error` on failure, such as HTTP errors
	///   or deserialization issues.
	///
	/// # Errors
	/// This function will return an `Error` if:
	/// - The underlying HTTP request fails.
	/// - The response from Salesforce is invalid or cannot be deserialized into the expected format.
	/// - Key or client issues arise when communicating with the Salesforce API.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{RestApi, SalesforceCredentials};
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let credentials = SalesforceCredentials::sfdx_url("force://PlatformCLI::token@example.com").unwrap();
	///     let api = RestApi::new(credentials).await.unwrap();
	///
	///     let object_name = "Account";
	///     let ids = vec!["001XXXXXXXXXXXXXXX", "001YYYYYYYYYYYYYYY"];
	///     let fields = vec!["Id", "Name"];
	///
	///     match api.sobject_by_ids::<serde_json::Value>(object_name, &ids, &fields).await {
	///         Ok(accounts) => {
	///             for account in accounts {
	///                 println!("{:?}", account);
	///             }
	///         },
	///         Err(e) => println!("Failed {:?}", e),
	///     }
	/// }
	/// ```
	///
	/// # Notes
	/// - This function utilizes Salesforce's composite API to efficiently fetch multiple records in a single request.
	/// - Ensure that the `fields` parameter only includes fields accessible to your API user based on their permissions.
	///
	/// # Related
	/// Consider using other methods to fetch Salesforce objects if you need different querying capabilities,
	/// such as querying with SOQL or retrieving data with additional filters.
	///
	/// # See
	/// https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_composite_sobjects_collections_retrieve.htm
	pub async fn sobject_by_ids<T: Debug + DeserializeOwned>(
		&self,
		object_name: &str,
		ids: &[&str],
		fields: &[&str],
	) -> Result<Vec<T>, Error> {
		self.build()
			.composite()
			.sobject_collections()
			.get_by_ids(object_name, ids, fields)
			.send()
			.await
	}
}

impl<'a> SObjectCollectionsBuilder<'a> {

	pub fn get_by_ids(
		&self,
		object_name: &str,
		ids: &[&str],
		fields: &[&str],
	) -> SObjectsByIdBuilder<'_, '_> {
		SObjectsByIdBuilder::new(self, object_name, ids, fields)
	}
}

pub struct SObjectsByIdBuilder<'builder, 'api> {
	builder: &'builder SObjectCollectionsBuilder<'api>,
	sobject_type: String,
	ids: Vec<String>,
	fields: Vec<String>,
}

impl<'builder, 'api>
SObjectsByIdBuilder<'builder, 'api> {

	pub(crate) fn new(
		builder: &'builder SObjectCollectionsBuilder<'api>,
		sobject_type : &str,
		ids: &[&str],
		fields: &[&str],
	) -> SObjectsByIdBuilder<'builder, 'api> {
		SObjectsByIdBuilder {
			builder,
			sobject_type: sobject_type.to_string(),
			ids: ids.iter().map(|id| id.to_string()).collect(),
			fields: fields.iter().map(|field| field.to_string()).collect(),
		}
	}

	pub async fn send<T: Debug + DeserializeOwned>(
		&self,
	) -> Result<Vec<T>, Error> {

		let params = vec![("fields", self.fields.join(",")), ("ids", self.ids.join(","))];
		let response = self.builder
			.send(HTTPMethod::GET,
			      self.sobject_type.clone(),
			      &params,
			)
			.await?;

		// fixme - why not strait to the response but first go through option? Could there be a none in the vector?
		let responses: Vec<Option<T>> = RestApi::handle_json_response(response).await?;
		Ok(responses.into_iter().filter_map(|r| r).collect())
	}
}
