use serde::Deserialize;

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

