use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SObjectErrorResponse {
	pub status_code: String,
	pub message: String,
	pub fields: Vec<String>,
}

impl std::fmt::Display for SObjectErrorResponse {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if !self.fields.is_empty() {
			write!(f, "Error: {} - {} (Fields: {:?})", self.status_code, self.message, self.fields)
		} else {
			write!(f, "Error: {} - {}", self.status_code, self.message)
		}
	}
}
