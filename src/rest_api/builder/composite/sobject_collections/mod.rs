//! # Composite SObjects Collections API
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
use crate::errors::Error;
use crate::rest_api::HTTPMethod;
use crate::rest_api::builder::composite::CompositeBuilder;
use reqwest::Response;
use serde::Serialize;
use std::fmt::Debug;

const PATH: &'static str = "sobjects";

mod create;
mod get_by_id;

impl<'a> CompositeBuilder<'a> {
	pub fn sobject_collections(&'a self) -> SObjectCollectionsBuilder<'a> {
		SObjectCollectionsBuilder::new(&self)
	}
}

pub struct SObjectCollectionsBuilder<'a> {
	builder: &'a CompositeBuilder<'a>,
}

impl<'a> SObjectCollectionsBuilder<'a> {
	pub fn new(builder: &'a CompositeBuilder) -> Self {
		SObjectCollectionsBuilder { builder }
	}

	/*
		pub(crate) async fn send_delete(
			&self,
			url: String,
		) -> anyhow::Result<Response, RestApiError> {
			let url = format!("{}/{}", PATH, url);
			self.builder.send_delete(url).await
		}
	*/
	pub(crate) async fn send<T: Debug + Serialize>(
		&self,
		method: HTTPMethod,
		url: String,
		data: &T,
	) -> Result<Response, Error> {
		let url = format!("{}/{}", PATH, url);
		self.builder.send(method, url, &data).await
	}
}
