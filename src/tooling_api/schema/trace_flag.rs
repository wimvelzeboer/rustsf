use crate::DefSObject;

use crate as rustsf;

#[DefSObject(sobject_type = "ApexLog")]
pub struct TraceFlag {
    #[serde(rename = "ApexCode")]
    pub apex_code: String,

    #[serde(rename = "ApexProfiling")]
    pub apex_profiling: String,

    #[serde(rename = "Callout")]
    pub callout: String,

    #[serde(rename = "CreatedById")]
    pub created_by_id: String,

    #[serde(rename = "CreatedDate")]
    pub created_date: String,

    #[serde(rename = "DataAccess")]
    pub data_access: String,

    #[serde(rename = "Database")]
    pub database: String,

    #[serde(rename = "DebugLevelId")]
    pub debug_level_id: String,

    #[serde(rename = "ExpirationDate")]
    pub expiration_date: String,

    #[serde(rename = "LogType")]
    pub log_type: String,

    #[serde(rename = "Nba")]
    pub nba: String,

    #[serde(rename = "StartDate")]
    pub start_date: String,

    #[serde(rename = "System")]
    pub system: String,

    #[serde(rename = "TracedEntityId")]
    pub traced_entity_id: String,

    #[serde(rename = "Validation")]
    pub validation: String,

    #[serde(rename = "Visualforce")]
    pub visualforce: String,

    #[serde(rename = "Wave")]
    pub wave: String,

    #[serde(rename = "Workflow")]
    pub workflow: String,
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
