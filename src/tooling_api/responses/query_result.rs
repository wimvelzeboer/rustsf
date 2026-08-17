use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::rest_api::responses::sobject_record::SObjectRecord;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryResult {

    /// Whether the query was successful
    pub done: bool,

    /// Compilation error message, if any.
    #[serde(rename = "entityTypeName")]

    pub entity_type_name: String,

    #[serde(rename = "queryLocator")]
    pub query_locator: Value,  // todo

    pub records: Vec<Value>,

    /// Whether the Apex code executed without runtime errors.
    pub success: bool,
    /// Line number of the error (-1 if no error).
    pub line: i32,
    /// Column number of the error (-1 if no error).
    pub column: i32,
    /// Runtime exception message, if any.
    pub exception_message: Option<String>,
    /// Runtime exception stack trace, if any.
    pub exception_stack_trace: Option<String>,
}