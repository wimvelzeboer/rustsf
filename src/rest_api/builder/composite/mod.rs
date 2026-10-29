use crate::errors::Error;
use crate::rest_api::HTTPMethod;
use crate::rest_api::builder::RestApiBuilder;
use reqwest::Response;
use serde::Serialize;
use std::fmt::Debug;

const PATH: &'static str = "composite";

mod sobject_collections;

impl<'a> RestApiBuilder<'a> {
	pub fn composite(&'a self) -> CompositeBuilder<'a> {
		CompositeBuilder::new(&self)
	}
}

pub struct CompositeBuilder<'a> {
	builder: &'a RestApiBuilder<'a>,
}

impl<'a> CompositeBuilder<'a> {
	pub fn new(builder: &'a RestApiBuilder) -> Self {
		CompositeBuilder { builder }
	}

	/*
		pub(crate) async fn send_delete(
			&self,
			url: String,
		) -> Result<Response, RestApiError> {
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
