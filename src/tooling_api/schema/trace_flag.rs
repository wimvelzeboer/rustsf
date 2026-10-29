use serde::{Deserialize, Serialize};

use crate as rustsf;
use rustsf::tooling_api::primary_types::log_category_level::LogCategoryLevel;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TraceFlag {
	#[serde(rename = "Id")]
	pub id: Option<String>,

	#[serde(rename = "ApexCode", skip_serializing)]
	pub apex_code: Option<LogCategoryLevel>,

	#[serde(rename = "ApexProfiling", skip_serializing)]
	pub apex_profiling: Option<LogCategoryLevel>,

	#[serde(rename = "Callout", skip_serializing)]
	pub callout: Option<LogCategoryLevel>,

	#[serde(rename = "DataAccess", skip_serializing)]
	pub data_access: Option<LogCategoryLevel>,

	#[serde(rename = "Database", skip_serializing)]
	pub database: Option<LogCategoryLevel>,

	#[serde(rename = "DebugLevelId")]
	pub debug_level_id: Option<String>,

	#[serde(rename = "ExpirationDate")]
	pub expiration_date: Option<String>,

	#[serde(rename = "LogType", skip_serializing)]
	pub log_type: Option<String>,

	#[serde(rename = "Nba", skip_serializing)]
	pub nba: Option<LogCategoryLevel>,

	#[serde(rename = "StartDate")]
	pub start_date: Option<String>,

	#[serde(rename = "System", skip_serializing)]
	pub system: Option<LogCategoryLevel>,

	#[serde(rename = "TracedEntityId", skip_serializing)]
	pub traced_entity_id: Option<String>,

	#[serde(rename = "Validation", skip_serializing)]
	pub validation: Option<LogCategoryLevel>,

	#[serde(rename = "Visualforce", skip_serializing)]
	pub visualforce: Option<LogCategoryLevel>,

	#[serde(rename = "Wave", skip_serializing)]
	pub wave: Option<LogCategoryLevel>,

	#[serde(rename = "Workflow", skip_serializing)]
	pub workflow: Option<LogCategoryLevel>,
}

impl TraceFlag {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn id(&self) -> Option<&str> {
		self.id.as_deref()
	}
	pub fn set_id(&mut self, id: Option<String>) {
		self.id = id;
	}
}

pub const SOBJECT_NAME: &'static str = "TraceFlag";

pub const FIELD_NAMES: &'static [&'static str] = &[
	"ApexCode",
	"ApexProfiling",
	"Callout",
	"CreatedById",
	"CreatedDate",
	"DataAccess",
	"Database",
	"DebugLevelId",
	"ExpirationDate",
	"Id",
	"LogType",
	"Nba",
	"StartDate",
	"System",
	"TracedEntityId",
	"Validation",
	"Visualforce",
	"Wave",
	"Workflow",
];
