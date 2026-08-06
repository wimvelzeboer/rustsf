use serde::{Deserialize, Serialize};

/// Represents an error response structure typically used in APIs to convey error details.
///
/// # Attributes
///
/// * `message` - A `String` containing the error message, providing details about the error.
/// * `error_code` - A `String` representing a specific code associated with the type of error that occurred.
/// * `fields` - An optional vector of `String` values specifying the fields related to the error, if applicable.
///
/// # Derived Traits
///
/// This struct derives the following traits:
/// * `Default` - Provides a default implementation for the struct.
/// * `Serialize` - Enables serialization of the struct into formats such as JSON.
/// * `Deserialize` - Allows deserialization of the struct from formats such as JSON.
/// * `Debug` - Provides formatting for debugging purposes.
/// * `Clone` - Allows cloning of the struct to create a duplicate instance.
///
/// # Serde Attributes
///
/// * `rename_all = "camelCase"` - Ensures that the serialized and deserialized field names use camelCase formatting, conforming to common JSON naming conventions.
///
/// # Example
/// ```
/// use your_crate::ErrorResponse;
///
/// let error = ErrorResponse {
///     message: "Invalid input".to_string(),
///     error_code: "INVALID_INPUT".to_string(),
///     fields: Some(vec!["username".to_string(), "password".to_string()]),
/// };
///
/// println!("{:?}", error);
/// ```
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub message: String,
    pub error_code: String,
    pub fields: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize() {
        let json_str = json!({
            "message": "Record not found",
            "errorCode": "NOT_FOUND",
            "fields": ["Id"]
        })
        .to_string();

        let resp: ErrorResponse = serde_json::from_str(&json_str).unwrap();
        assert_eq!(resp.message, "Record not found");
        assert_eq!(resp.error_code, "NOT_FOUND");
        assert_eq!(resp.fields, Some(vec!["Id".to_string()]));
    }

    #[test]
    fn test_without_fields() {
        let json_str = json!({
            "message": "Error",
            "errorCode": "ERR"
        })
        .to_string();

        let resp: ErrorResponse = serde_json::from_str(&json_str).unwrap();
        assert_eq!(resp.fields, None);
    }

    #[test]
    fn test_default() {
        let resp = ErrorResponse::default();
        assert_eq!(resp.message, "");
        assert_eq!(resp.error_code, "");
        assert_eq!(resp.fields, None);
    }
}
