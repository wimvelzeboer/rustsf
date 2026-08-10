
#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct SObjectAttribute {
    #[serde(rename = "type")]
    pub sobject_type: String,
    pub url: String,
}

impl SObjectAttribute {
    pub fn new(sobject_type: &str) -> Self {
        let url = format!("/services/data/v67.0/sobjects/{}/describe", sobject_type);
        Self {
            sobject_type: sobject_type.to_string(),
            url,
        }
    }

    pub fn set_id(&mut self, id: Option<&str>) -> &mut Self {
        match id {
            Some(id) => {
                self.url = format!("{}/{}", self.url, id);
            },
            None => {
                self.url =  format!("/services/data/v67.0/sobjects/{}/describe", self.sobject_type);
            },
        }
        self
    }
}