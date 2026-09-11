use self::sobjects::SObjectsBuilder;
use crate::RestApi;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;

mod headers;
mod sobjects;

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

	pub(crate) async fn post<T: Debug + Serialize>(&self, url: String, data: &T) -> anyhow::Result<reqwest::Response> {
		let url = format!("services/data/{}/{}", self.rest_api.client.api_version(), url);
		self.rest_api.post(url, &data, &self.headers).await
	}
}
