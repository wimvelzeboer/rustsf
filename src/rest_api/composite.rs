//! # Composite SObjects API
//!
//! The Composite SObjects API allows you to create multiple records in Salesforce
//! using a single HTTP request. This can be useful for bulk operations, such as
//! creating multiple accounts or contacts, up to 200 records, in a single API call.
//!
//! ## Supported Endpoints
//! - [**/services/data/vXX.X/composite/sobjects**](crate::rest_api::composite),
//!
//! ## Methods
//!
//! - [**create**](crate::rest_api::RestApi#method.create), creates a batch of records in Salesforce using the Composite SObjects API.
//! - [**sobject_by_ids**](crate::rest_api::RestApi#method.sobject_by_ids), retrieves multiple records from a Salesforce object by their IDs.
//!
//! # See
//! <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_composite_sobjects_collections.htm>
//!
//!
use crate::primary_types::{SObject, SObjectOwner};
use crate::rest_api::responses::sobject_create_request::SObjectCreateRequest;
use crate::rest_api::responses::sobject_create_response::SObjectCreateResponse;
use crate::rest_api::{RestApi, handle_json_response};
use anyhow::{Context, Result, anyhow};
use log::debug;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

impl RestApi {

	pub async fn sobject_by_ids<T: Serialize + DeserializeOwned + Debug + SObject>(
		&mut self,
		object_name: &str,
		ids: Vec<&str>,
		fields: Vec<&str>,
	) -> Result<Vec<T>> {
		let url = format!(
			"{}/composite/sobjects/{}",
			self.client.base_version_path()?,
			object_name
		);
		let params = vec![("fields", fields.join(",")), ("ids", ids.join(","))];
		let response = self.client.post(url, params, vec![]).await?;
		let responses: Vec<Option<T>> = handle_json_response(response).await?;
		Ok(responses.into_iter().filter_map(|r| r).collect())
	}
}
