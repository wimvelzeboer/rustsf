//! # SObjects API
//!
//! Module containing all the sObject Basic Information and CRUD operations on a single Salesforce object
//!
//! ## Supported Endpoints
//! - **/services/data/vXX.X/sobjects/**
//!
//! ## Methods
//! - [create_sobject](crate::rest_api::RestApi#method.create_sobject), Creates a new SObject record.
//! - [update_sobject](crate::rest_api::RestApi#method.update_sobject), Updates a single SObject record.
//! - [upsert_sobject](crate::rest_api::RestApi#method.upserts_sobject), Creates or updates a single SObject record.
//! - [delete_sobject](crate::rest_api::RestApi#method.delete_sobject), Deletes a specific SObject record by its ID.
//! - [sobject_basic_info](crate::rest_api::RestApi#method.sobject_basic_info), Basic information for a specified SObjectType.
//! - [describe_sobject](crate::rest_api::RestApi#method.describe_sobject), Detailed information about a specific SObject.
//! - [describe_global](crate::rest_api::RestApi#method.describe_global), Retrieves a list of all global SObjects.
//! - [describe_global_modified](crate::rest_api::RestApi#method.describe_global_modified), Retrieves a list of all global SObjects modified since a given date.
//! - [describe_global_unmodified](crate::rest_api::RestApi#method.describe_global_unmodified), Retrieves a list of all global SObjects not modified since a given date.

//! - [fetch_by_id](crate::rest_api::RestApi#method.fetch_by_id), Retrieves a specific SObject record by its ID.
//! - [sobject_get_deleted](crate::rest_api::RestApi#method.sobject_get_deleted), Retrieves a list of deleted SObjects.
//! - [sobject_get_updated](crate::rest_api::RestApi#method.sobject_get_updated), Retrieves a list of updated SObjects.
//!
//! # See
//! <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_basic_info.htm>

use super::RestApiBuilder;
use crate::rest_api::HTTPMethod;
use anyhow::Result;
use reqwest::Response;
use serde::{Serialize};
use std::fmt::Debug;
use crate::rest_api::error::RestApiError;

mod create;
mod delete;
mod deleted;

mod global_describe;
mod sobject_basic_info;
mod sobject_by_id;
mod sobject_by_ext_id;
mod sobject_describe;

mod update;
mod updated;
mod upsert_by_ext_id;
mod user_password;

impl<'a> RestApiBuilder<'a> {
	pub fn sobjects(&'a self) -> SObjectsBuilder<'a> {
		SObjectsBuilder::new(&self)
	}
}

pub struct SObjectsBuilder<'a> {
	builder: &'a RestApiBuilder<'a>,
}

impl<'a> SObjectsBuilder<'a> {
	pub fn new(builder: &'a RestApiBuilder) -> Self {
		SObjectsBuilder { builder }
	}

	pub(crate) async fn send_delete(
		&self,
		url: String,
	) -> Result<Response, RestApiError> {
		let url = format!("sobjects/{}", url);
		self.builder.send_delete(url).await
	}

	pub(crate) async fn send<T: Debug + Serialize>(
		&self,
		method: HTTPMethod,
		url: String,
		data: &T,
	) -> Result<Response, RestApiError> {
		let url = format!("sobjects/{}", url);
		self.builder.send(method, url, &data).await
	}
}