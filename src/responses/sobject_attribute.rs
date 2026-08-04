use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SObjectAttribute {
    #[serde(rename = "type")]
    pub sobject_type: String,
    pub url: String,
}