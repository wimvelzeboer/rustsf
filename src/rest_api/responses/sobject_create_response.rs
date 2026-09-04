use super::sobject_error_response::SObjectErrorResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SObjectCreateResponse {
	pub id: Option<String>,
	pub success: bool,
	pub errors: Vec<SObjectErrorResponse>,
}

impl SObjectCreateResponse {
	pub fn id(&self) -> Option<&str> {
		self.id.as_deref()
	}

	pub fn is_success(&self) -> bool {
		self.success
	}
}

impl std::fmt::Display for SObjectCreateResponse {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.success {
			write!(f, "Success: {}", self.id.as_ref().unwrap_or(&String::new()))
		} else {
			write!(
				f,
				"SObject creation failed: {}",
				self.errors
					.iter()
					.map(|error| error.to_string())
					.collect::<Vec<String>>()
					.join(", ")
			)
		}
	}
}
