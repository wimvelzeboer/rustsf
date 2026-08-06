use serde::Deserialize;
use super::child_relationship::ChildRelationship;
use super::field::Field;
use super::urls::Urls;

///
/// Struct representing the response from a "Describe" API call.
///
/// This struct is used to deserialize JSON responses into a strongly-typed format.
/// It contains metadata about a given object, including its fields, child relationships,
/// and capabilities such as create, update, delete, and query options.
///
/// The `serde` attributes allow for flexible serialization and deserialization, including
/// handling of default values and camel-case field name matching.
///
/// # Fields
///
/// * `activateable` - Boolean flag indicating if the object is activatable. Defaults to `false`.
/// * `child_relationships` - A vector of child relationship metadata associated with this object.
/// * `compact_layoutable` - Boolean flag indicating if the object supports compact layouts. Defaults to `false`.
/// * `createable` - Boolean flag indicating if the object supports creation via the API. Defaults to `false`.
/// * `custom` - Boolean flag indicating if the object is a custom object. Defaults to `false`.
/// * `custom_setting` - Boolean flag indicating if the object is a custom setting. Defaults to `false`.
/// * `deletable` - Boolean flag indicating if the object can be deleted via the API. Defaults to `false`.
/// * `deprecated_and_hidden` - Boolean flag indicating if the object is deprecated and hidden. Defaults to `false`.
/// * `feed_enabled` - Boolean flag indicating if feed tracking is enabled for the object. Defaults to `false`.
/// * `fields` - A vector of field metadata associated with the object.
/// * `has_subtypes` - Boolean flag indicating if the object has subtypes. Defaults to `false`.
/// * `is_subtype` - Boolean flag indicating if the object is a subtype. Defaults to `false`.
/// * `key_prefix` - Optional string representing the key prefix for the object's records.
/// * `label` - The singular label for the object.
/// * `label_plural` - The plural label for the object.
/// * `layoutable` - Boolean flag indicating if layouts are supported for the object. Defaults to `false`.
/// * `listviewable` - Optional boolean flag indicating if the object is list-viewable.
/// * `lookup_layoutable` - Optional boolean flag indicating if the object supports lookup layouts.
/// * `mergeable` - Boolean flag indicating if the object can participate in merges. Defaults to `false`.
/// * `mru_enabled` - Boolean flag indicating if the object supports "most recently used" (MRU) functionality. Defaults to `false`.
/// * `name` - The API name of the object.
/// * `queryable` - Boolean flag indicating if the object can be queried via the API. Defaults to `false`.
/// * `replicateable` - Boolean flag indicating if the object supports replication. Defaults to `false`.
/// * `retrieveable` - Boolean flag indicating if the object can be retrieved via the API. Defaults to `false`.
/// * `search_layoutable` - Boolean flag indicating if the object supports search layouts. Defaults to `false`.
/// * `searchable` - Boolean flag indicating if the object is searchable via a global search. Defaults to `false`.
/// * `triggerable` - Boolean flag indicating if triggers are supported on the object. Defaults to `false`.
/// * `undeletable` - Boolean flag indicating if the object cannot be deleted. Defaults to `false`.
/// * `updateable` - Boolean flag indicating if the object supports update operations via the API. Defaults to `false`.
/// * `urls` - A struct containing various URLs associated with the object, such as links to its metadata or data.
///
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