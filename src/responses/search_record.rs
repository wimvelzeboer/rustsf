use serde::Deserialize;
use crate::responses::sobject_attribute::SObjectAttribute;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchRecord {
    #[serde(rename = "Id")]
    pub id: String,
    pub attributes: SObjectAttribute,
}