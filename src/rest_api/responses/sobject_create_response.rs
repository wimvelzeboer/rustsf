use serde::{Deserialize, Serialize};
use crate::primary_types::Id18;
use super::sobject_error_response::SObjectErrorResponse;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SObjectCreateResponse {
    pub id: String,
    pub success: bool,
    pub errors: Vec<SObjectErrorResponse>,
}