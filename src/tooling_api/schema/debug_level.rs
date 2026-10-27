use serde::{Deserialize, Serialize};
use crate as rustsf;
use rustsf::tooling_api::primary_types::log_category_level::LogCategoryLevel;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugLevel {

	#[serde(rename = "Id")]
	pub id: Option<String>,

	#[serde(rename = "ApexCode")]
	pub apex_code: LogCategoryLevel,

	#[serde(rename = "ApexProfiling")]
	pub apex_profiling: LogCategoryLevel,

	#[serde(rename = "Callout")]
	pub callout: LogCategoryLevel,

	#[serde(rename = "Database")]
	pub database: LogCategoryLevel,

	#[serde(rename = "DeveloperName")]
	pub developer_name: String,

	#[serde(rename = "Language")]
	pub language: Option<String>,

	#[serde(rename = "MasterLabel")]
	pub master_label: String,

	#[serde(rename = "System")]
	pub system: LogCategoryLevel,

	#[serde(rename = "Validation")]
	pub validation: LogCategoryLevel,

	#[serde(rename = "Visualforce")]
	pub visualforce: LogCategoryLevel,

	#[serde(rename = "Workflow")]
	pub workflow: LogCategoryLevel,
}

impl DebugLevel {

	pub fn id(&self) -> Option<&str> {
		self.id.as_deref()
	}
}

pub const SOBJECT_NAME: &'static str = "DebugLevel";

pub const FIELD_NAMES: &'static [&'static str] = &[
	"ApexCode",
	"ApexProfiling",
	"Callout",
	"Database",
	"DeveloperName",
	"Id",
	"Language",
	"MasterLabel",
	"System",
	"Validation",
	"Visualforce",
	"Workflow",
];
