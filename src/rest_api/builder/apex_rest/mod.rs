use crate::rest_api::HTTPMethod;
use crate::{Error, RestApi};
use std::collections::HashMap;
use std::fmt::Debug;
use reqwest::Response;
use serde::Serialize;

impl RestApi {
	pub async fn apex_rest<T: Debug + Serialize>(
		&self,
		method: HTTPMethod,
		url: &str,
		data: &T,
		headers: &HashMap<String, String>,
	) -> Result<Response, Error> {
		let url = format!("services/apexrest/{}", url);

		self.send(
			method,
			url,
			data,
			headers,
		).await
	}
}
