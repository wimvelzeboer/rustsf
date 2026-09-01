use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseError {
	pub message: String,
}

impl ResponseError {
	pub fn new(message: String) -> Self {
		Self { message }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;

	#[test]
	fn test_deserialize() {
		let json_str = json!({
			"message": "Record not found",
		})
		.to_string();

		let resp: ResponseError = serde_json::from_str(&json_str).unwrap();
		assert_eq!(resp.message, "Record not found");
	}

	#[test]
	fn test_default() {
		let resp = ResponseError::default();
		assert_eq!(resp.message, "");
	}
}
