use super::sobject_error_response::SObjectErrorResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SObjectCreateResponse {
	pub id: String,
	pub success: bool,
	pub errors: Vec<SObjectErrorResponse>,
}
