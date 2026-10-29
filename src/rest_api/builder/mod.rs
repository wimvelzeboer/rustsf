use crate::RestApi;
use crate::errors::Error;
use crate::rest_api::HTTPMethod;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;

mod composite;
mod headers;
mod limits;
mod query;
mod search;
// mod search;
mod sobjects;
mod versions;

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

	pub(crate) async fn send_delete(&self, url: String) -> Result<reqwest::Response, Error> {
		let url = format!("services/data/v{}/{}", crate::rest_api::DEFAULT_API_VERSION, url);
		self.rest_api.send_delete(url, &self.headers).await
	}

	pub(crate) async fn send<T: Debug + Serialize>(
		&self,
		method: HTTPMethod,
		url: String,
		data: &T,
	) -> Result<reqwest::Response, Error> {
		let url = format!("services/data/v{}/{}", crate::rest_api::DEFAULT_API_VERSION, url);
		self.rest_api.send(method, url, &data, &self.headers).await
	}
}
