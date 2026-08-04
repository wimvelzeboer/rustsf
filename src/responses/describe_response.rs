use serde::Deserialize;
use crate::responses::child_relationship::ChildRelationship;
use crate::responses::field::Field;
use crate::responses::urls::Urls;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DescribeResponse {
    #[serde(default)]
    pub activateable: bool,
    //    pub action_overrides: ActionOverride[],
    pub child_relationships: Vec<ChildRelationship>,
    #[serde(default)]
    pub compact_layoutable: bool,
    #[serde(default)]
    pub createable: bool,
    #[serde(default)]
    pub custom: bool,
    #[serde(default)]
    pub custom_setting: bool,
    #[serde(default)]
    pub deletable: bool,
    #[serde(default)]
    pub deprecated_and_hidden: bool,
    #[serde(default)]
    pub feed_enabled: bool,
    #[serde(default)]
    pub fields: Vec<Field>,
    #[serde(default)]
    pub has_subtypes: bool,
    #[serde(default)]
    pub is_subtype: bool,
    pub key_prefix: Option<String>,
    pub label: String,
    pub label_plural: String,

    #[serde(default)]
    pub layoutable: bool,
    pub listviewable: Option<bool>,
    pub lookup_layoutable: Option<bool>,

    #[serde(default)]
    pub mergeable: bool,

    #[serde(default)]
    pub mru_enabled: bool,
    pub name: String,
    //    pub named_layout_infos: [],
    //    pub network_scope_field_name: [],

    #[serde(default)]
    pub queryable: bool,
    //    pub record_type_infos: Record_type_info[]

    #[serde(default)]
    pub replicateable: bool,

    #[serde(default)]
    pub retrieveable: bool,

    #[serde(default)]
    pub search_layoutable: bool,

    #[serde(default)]
    pub searchable: bool,
    //    pub supported_scopes:  Scope_info

    #[serde(default)]
    pub triggerable: bool,

    #[serde(default)]
    pub undeletable: bool,

    #[serde(default)]
    pub updateable: bool,
    pub urls: Urls,
}