use std::fmt::Debug;
use reqwest::Response;
use serde::Serialize;
use crate::rest_api::builder::composite::CompositeBuilder;
use crate::rest_api::builder::RestApiBuilder;
use crate::rest_api::HTTPMethod;

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

	pub(crate) async fn send_delete(
		&self,
		url: String,
	) -> anyhow::Result<Response> {
		let url = format!("{}/{}", PATH, url);
		self.builder.send_delete(url).await
	}

	pub(crate) async fn send<T: Debug + Serialize>(
		&self,
		method: HTTPMethod,
		url: String,
		data: &T,
	) -> anyhow::Result<Response> {
		let url = format!("{}/{}", PATH, url);
		self.builder.send(method, url, &data).await
	}
}