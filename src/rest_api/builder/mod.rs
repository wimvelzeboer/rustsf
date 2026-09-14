use crate::RestApi;
use crate::rest_api::HTTPMethod;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;

mod headers;
mod sobjects;
pub mod versions;

impl RestApi {
	pub fn build(&self) -> RestApiBuilder<'_> {
		RestApiBuilder::new(&self)
	}
}

pub struct RestApiBuilder<'a> {
	rest_api: &'a RestApi,
	headers: HashMap<String, String>,
}

impl<'a> RestApiBuilder<'a> {
	pub fn new(rest_api: &'a RestApi) -> Self {
		Self {
			rest_api,
			headers: HashMap::new(),
		}
	}

	pub(crate) async fn send_delete(
		&self,
		url: String,
	) -> anyhow::Result<reqwest::Response> {
		let url = format!("services/data/{}/{}", self.rest_api.client.api_version(), url);
		self.rest_api.send_delete(url, &self.headers).await
	}

	pub(crate) async fn send<T: Debug + Serialize>(
		&self,
		method: HTTPMethod,
		url: String,
		data: &T,
	) -> anyhow::Result<reqwest::Response> {
		let url = format!("services/data/{}/{}", self.rest_api.client.api_version(), url);
		self.rest_api.send(method, url, &data, &self.headers).await
	}
}
