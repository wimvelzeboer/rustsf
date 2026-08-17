use crate::DefSObject;

use crate as rustsf;
use rustsf::tooling_api::primary_types::log_category_level::LogCategoryLevel;

#[DefSObject(sobject_type = "ApexLog")]
pub struct TraceFlag {
    #[serde(rename = "ApexCode")]
    pub apex_code: LogCategoryLevel,

    #[serde(rename = "ApexProfiling")]
    pub apex_profiling: LogCategoryLevel,

    #[serde(rename = "Callout")]
    pub callout: String,

    #[serde(rename = "CreatedById")]
    pub created_by_id: String,

    #[serde(rename = "CreatedDate")]
    pub created_date: String,

    #[serde(rename = "DataAccess")]
    pub data_access: LogCategoryLevel,

    #[serde(rename = "Database")]
    pub database: LogCategoryLevel,

    #[serde(rename = "DebugLevelId")]
    pub debug_level_id: String,

    #[serde(rename = "ExpirationDate")]
    pub expiration_date: String,

    #[serde(rename = "LogType")]
    pub log_type: String,

    #[serde(rename = "Nba")]
    pub nba: LogCategoryLevel,

    #[serde(rename = "StartDate")]
    pub start_date: String,

    #[serde(rename = "System")]
    pub system: LogCategoryLevel,

    #[serde(rename = "TracedEntityId")]
    pub traced_entity_id: String,

    #[serde(rename = "Validation")]
    pub validation: LogCategoryLevel,

    #[serde(rename = "Visualforce")]
    pub visualforce: LogCategoryLevel,

    #[serde(rename = "Wave")]
    pub wave: LogCategoryLevel,

    #[serde(rename = "Workflow")]
    pub workflow: LogCategoryLevel,
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
