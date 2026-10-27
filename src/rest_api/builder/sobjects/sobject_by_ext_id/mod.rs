use serde::de::DeserializeOwned;
use crate::rest_api::builder::sobjects::SObjectsBuilder;
use crate::errors::Error;
use crate::rest_api::HTTPMethod;
use crate::RestApi;

impl RestApi {

	/// Retrieves a record based on the value of the specified external ID field.
	///
	/// # See
	/// https://developer.salesforce.com/docs/platform/api-rest/guide/using-resources-retrieve-with-externalid.html
	pub async fn sobject_by_ext_id<'a, T: DeserializeOwned>(
		&self,
		sobject_name: &str,
		ext_id_field: &str,
		ext_id: &str,
	) -> Result<T, Error> {
		self.build()
			.sobjects()
			.get_by_ext_id(sobject_name, ext_id_field, ext_id)
			.send()
			.await
	}
}

impl<'a> SObjectsBuilder<'a> {

	/// Retrieves a record based on the value of the specified external ID field.
	///
	/// # See
	/// https://developer.salesforce.com/docs/platform/api-rest/guide/resources-sobject-upsert-get.html
	pub fn get_by_ext_id(
		&self,
		sobject_name: &str,
		ext_id_field: &str,
		ext_id: &str,
	) -> SObjectSelectByExtIdBuilder<'_> {
		SObjectSelectByExtIdBuilder::new(self, sobject_name, ext_id_field, ext_id)
	}
}

pub struct SObjectSelectByExtIdBuilder<'a> {
	builder: &'a SObjectsBuilder<'a>,
	sobject_type: String,
	ext_id_field: String,
	ext_id: String,
}

impl<'a> SObjectSelectByExtIdBuilder<'a> {
	pub(crate) fn new(builder: &'a SObjectsBuilder,
	                  sobject_name: &str,
	                  ext_id_field: &str,
	                  ext_id: &str,
	) -> SObjectSelectByExtIdBuilder<'a> {
		Self {
			builder,
			sobject_type: sobject_name.to_string(),
			ext_id_field: ext_id_field.to_string(),
			ext_id: ext_id.to_string(),
		}
	}

	pub async fn send<T: DeserializeOwned>(
		&self,
	) -> Result<T, Error> {
		let param: Vec<(&str, &str)> = vec![];
		let response = self.builder.send(
			HTTPMethod::GET,
			format!("{}/{}/{}", self.sobject_type, self.ext_id_field, self.ext_id),
			&param,
		).await?;
		RestApi::handle_json_response(response).await
	}
}

