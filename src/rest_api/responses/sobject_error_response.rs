use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SObjectErrorResponse {
    pub status_code: String,
    pub message: String,
    pub fields: Vec<String>,
}