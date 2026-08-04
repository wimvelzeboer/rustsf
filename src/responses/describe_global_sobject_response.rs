use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DescribeGlobalSObjectResponse {
    pub activateable: bool,
    pub createable: bool,
    pub custom: bool,
    pub custom_setting: bool,
    pub deletable: bool,
    pub deprecated_and_hidden: bool,
    pub feed_enabled: bool,
    pub has_subtypes: bool,
    pub is_subtype: bool,
    pub key_prefix: Option<String>,
    pub label: String,
    pub label_plural: String,
    pub layoutable: bool,
    pub mergeable: bool,
    pub mru_enabled: bool,
    pub name: String,
    pub queryable: bool,
    pub replicateable: bool,
    pub retrieveable: bool,
    pub searchable: bool,
    pub triggerable: bool,
    pub undeletable: bool,
    pub updateable: bool,
    pub urls: HashMap<String, String>,
}