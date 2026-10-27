use anyhow::Result;
use rustsf::{RestApi, SalesforceCredentials};

#[cfg(feature = "metadata-api")]
use rustsf::MetadataApi;

#[cfg(feature = "tooling-api")]
use rustsf::ToolingApi;

use serde::{Deserialize, Serialize};
use std::env;

#[allow(dead_code)]
pub async fn get_metadata_api_client() -> Result<MetadataApi> {
	let url = env::var("SCRATCH_AUTH_URL")
		.expect("SCRATCH_AUTH_URL not set")
		.to_string();
	let credentials = SalesforceCredentials::sfdx_url(url)?;
	Ok(MetadataApi::new(credentials).await?)
}

#[allow(dead_code)]
#[cfg(feature = "rest-api")]
pub async fn get_rest_api_client() -> Result<RestApi> {
	let url = env::var("SCRATCH_AUTH_URL")
		.expect("SCRATCH_AUTH_URL not set")
		.to_string();
	let credentials = SalesforceCredentials::sfdx_url(url)?;
	Ok(RestApi::new(credentials).await?)
}

#[allow(dead_code)]
#[cfg(feature = "tooling-api")]
pub async fn get_tooling_api_client() -> Result<ToolingApi> {
	let url = env::var("SCRATCH_AUTH_URL")
		.expect("SCRATCH_AUTH_URL not set")
		.to_string();
	let credentials = SalesforceCredentials::sfdx_url(url)?;
	Ok(ToolingApi::new(credentials).await?)
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
	#[serde(rename = "type")]
	pub sobject_type: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
	pub attributes: Attribute,

	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,

	#[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,

	#[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,

	#[serde(rename = "OwnerId")]
	pub owner_id: Option<String>,
}

#[allow(dead_code)]
impl Account {
	pub fn new() -> Self {
		Self {
			attributes: Attribute {
				sobject_type: "Account".to_string(),
			},
			id: None,
			description: None,
			name: None,
			owner_id: None,
		}
	}

	pub fn set_name(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}

	pub fn set_owner_id(mut self, id: impl Into<String>) -> Self {
		self.owner_id = Some(id.into());
		self
	}

	pub fn get_name(&self) -> Option<&str> {
		self.name.as_deref()
	}
}
