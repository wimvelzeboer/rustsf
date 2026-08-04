use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VersionResponse {
    pub label: String,
    pub url: String,
    pub version: String,
}