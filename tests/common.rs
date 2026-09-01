use anyhow::Result;
use rustsf::{AuthUrl, Client, DefSObject, RestApi};
use std::env;

pub async fn get_rest_api_client() -> Result<RestApi> {
	let url = env::var("SCRATCH_AUTH_URL")
		.expect("SCRATCH_AUTH_URL not set")
		.to_string();
	let client = Client::new(AuthUrl::new(url)?).await?;
	Ok(RestApi::new(client))
}

#[DefSObject(sobject_type = "Account", fields = "name,owner")]
pub struct Account {}

impl Account {
	pub fn set_name(mut self, name: String) -> Self {
		self.name = Some(name);
		self
	}

	pub fn get_name(&self) -> Option<&str> {
		self.name.as_deref()
	}
}
