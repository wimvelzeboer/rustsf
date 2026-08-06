use serde::Deserialize;

/// Represents the response structure for a creation operation.
///
/// This struct is typically used to deserialize the response from an API or other data source
/// after creating a resource.
///
/// # Fields
///
/// * `id` - A `String` that uniquely identifies the created resource. This will usually be
///          a UUID or some other identifier assigned by the system.
///
/// * `success` - A `bool` indicating whether the creation was successful. A value of `true`
///               signifies that the resource was created successfully, while a value of `false`
///               indicates failure.
///
/// # Traits
///
/// This struct derives the following traits:
/// * `Deserialize` - Enables deserialization of data into this struct, making it compatible
///                   with formats such as JSON, TOML, etc.
/// * `Debug` - Allows for the struct to be debug-printed for easier logging and debugging.
///
#[derive(Deserialize, Debug)]
pub struct CreateResponse {
    pub id: String,
    pub success: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize() {
        let json_str = json!({
            "id": "0019K00001pVhatQAC",
            "success": true,
        })
            .to_string();

        let resp: CreateResponse = serde_json::from_str(&json_str).unwrap();
        assert_eq!(resp.id, "0019K00001pVhatQAC");
        assert_eq!(resp.success, true);
    }
}

