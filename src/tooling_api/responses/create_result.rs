use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateResult {
    pub errors: Vec<String>,
    pub id: String,
    pub infos: Vec<String>,
    pub success: bool,
    pub warnings: Vec<String>,
}