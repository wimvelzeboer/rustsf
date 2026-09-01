use crate::primary_types::Id18;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SObjectRecord {
	#[serde(rename = "attributes")]
	pub attributes: Attributes,
	pub id: Id18,
	pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
	#[serde(rename = "type")]
	pub object_type: String,
	pub url: String,
}

// todo - Add a template that adds the default fields to any existing struct
