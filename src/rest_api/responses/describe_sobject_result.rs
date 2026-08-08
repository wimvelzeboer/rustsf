use super::action_override::ActionOverride;
use crate::rest_api::responses::child_relationship::ChildRelationship;
use crate::rest_api::responses::field::Field;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Represents metadata returned by Salesforce for a described SObject.
///
/// This struct is used to deserialize JSON responses from the Salesforce
/// Describe SObject API. The response contains object-level metadata,
/// relationship metadata, field metadata, URL templates, permissions,
/// layout capabilities, record type information, and other describe details.
///
/// # See
/// <https://developer.salesforce.com/docs/atlas.en-us.api.meta/api/sforce_api_calls_describesobjects_describesobjectresult.htm>
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeSObjectResult {
    /// Reserved for future use.
    ///
    /// Salesforce currently exposes this flag for compatibility and future
    /// platform functionality.
    #[serde(default)]
    pub activateable: bool,

    /// Metadata for action overrides defined on the object.
    ///
    /// Action overrides describe custom behavior for standard or custom actions,
    /// such as overriding the default New, Edit, View, Tab, or List actions.
    #[serde(default)]
    pub action_overrides: Vec<ActionOverride>,

    /// Metadata for child relationships from this object to other objects.
    ///
    /// Each child relationship describes how another object relates back to this
    /// object, including relationship names, child object names, cascade delete
    /// behavior, and the field used for the relationship.
    #[serde(default)]
    pub child_relationships: Vec<ChildRelationship>,

    /// Indicates whether the object can be used with compact layouts.
    ///
    /// If `true`, compact layout metadata can be retrieved for this object.
    #[serde(default)]
    pub compact_layoutable: bool,

    /// Indicates whether records of this object can be created.
    ///
    /// This value depends on the object type and the current user's permissions.
    #[serde(default)]
    pub createable: bool,

    /// Indicates whether this is a custom object.
    ///
    /// Custom objects usually have API names ending in `__c`.
    #[serde(default)]
    pub custom: bool,

    /// Indicates whether this object is a custom setting.
    ///
    /// Custom settings are custom objects used to store application configuration data.
    #[serde(default)]
    pub custom_setting: bool,

    /// Indicates whether records of this object can be deep-cloned.
    ///
    /// Deep cloning copies a record and certain related records, depending on
    /// Salesforce object support and user permissions.
    #[serde(default)]
    pub deep_cloneable: bool,

    /// Indicates whether records of this object can be deleted.
    ///
    /// This value depends on the object type and the current user's permissions.
    #[serde(default)]
    pub deletable: bool,

    /// Indicates whether this object is deprecated and hidden.
    ///
    /// Deprecated and hidden objects are retained for compatibility but should
    /// generally not be used for new development.
    #[serde(default)]
    pub deprecated_and_hidden: bool,

    /// Indicates whether Chatter feed tracking is enabled for this object.
    ///
    /// If `true`, the object supports feed-related functionality.
    #[serde(default)]
    pub feed_enabled: bool,

    /// Field metadata for this object.
    ///
    /// Each entry describes a field on the object, including its name, label,
    /// type, length, precision, relationship information, permission flags,
    /// picklist values, and other field-level metadata.
    #[serde(default)]
    pub fields: Vec<Field>,

    /// Indicates whether this object has subtypes.
    ///
    /// Some Salesforce objects can act as base objects with specialized subtypes.
    #[serde(default)]
    pub has_subtypes: bool,

    /// Indicates whether this object is an interface object.
    ///
    /// Interface objects represent common metadata shared by multiple concrete
    /// object implementations.
    #[serde(default)]
    pub is_interface: bool,

    /// Indicates whether this object is a subtype of another object.
    ///
    /// If `true`, the object represents a specialized subtype rather than a base object.
    #[serde(default)]
    pub is_subtype: bool,

    /// The three-character key prefix used in record IDs for this object.
    ///
    /// Standard and custom objects that have Salesforce record IDs generally have
    /// a key prefix. Some objects may not expose one, so this value is optional.
    #[serde(default)]
    pub key_prefix: Option<String>,

    /// The display label for this object.
    ///
    /// This is the singular, user-facing label shown in the Salesforce UI.
    #[serde(default)]
    pub label: String,

    /// The plural display label for this object.
    ///
    /// This is the plural, user-facing label shown in the Salesforce UI.
    #[serde(default)]
    pub label_plural: String,

    /// Indicates whether this object supports layouts.
    ///
    /// If `true`, layout metadata can be retrieved for this object.
    #[serde(default)]
    pub layoutable: bool,

    /// Indicates whether list views are supported for this object.
    ///
    /// This value can be absent for some objects or API versions, so it is optional.
    #[serde(default)]
    pub listviewable: Option<bool>,

    /// Indicates whether lookup layouts are supported for this object.
    ///
    /// Lookup layouts control which fields appear when records of this object are
    /// displayed in lookup dialogs.
    #[serde(default)]
    pub lookup_layoutable: Option<bool>,

    /// Indicates whether records of this object can be merged.
    ///
    /// This is typically available for selected standard objects such as Account,
    /// Contact, and Lead, depending on permissions and configuration.
    #[serde(default)]
    pub mergeable: bool,

    /// Indicates whether recently viewed records are tracked for this object.
    ///
    /// If `true`, the object can appear in most-recently-used lists.
    #[serde(default)]
    pub mru_enabled: bool,

    /// The API name of the object.
    ///
    /// For custom objects, this usually ends in `__c`.
    #[serde(default)]
    pub name: String,

    /// Metadata for named layouts associated with this object.
    ///
    /// Named layout information identifies special layouts that can be retrieved
    /// or referenced for the object.
    #[serde(default)]
    pub named_layout_infos: Vec<NamedLayoutInfo>,

    /// The field used to scope this object in Salesforce communities or networks.
    ///
    /// This value is only present for objects that support network scoping.
    #[serde(default)]
    pub network_scope_field_name: Option<String>,

    /// Indicates whether this object can be queried using SOQL.
    ///
    /// If `true`, records of this object can be queried by the current user.
    #[serde(default)]
    pub queryable: bool,

    /// Record type metadata for this object.
    ///
    /// Each entry describes a record type, including its ID, name, availability,
    /// default mapping, master status, and other record type details.
    #[serde(default)]
    pub record_type_infos: Vec<Value>,

    /// Indicates whether this object supports data replication calls.
    ///
    /// If `true`, the object can be used with replication-oriented APIs such as
    /// calls that retrieve updated or deleted records.
    #[serde(default)]
    pub replicateable: bool,

    /// Indicates whether records of this object can be retrieved.
    ///
    /// This value depends on object support and the current user's permissions.
    #[serde(default)]
    pub retrieveable: bool,

    /// Indicates whether search layouts are supported for this object.
    ///
    /// Search layouts determine which fields appear in search results and lookup
    /// dialogs for supported objects.
    #[serde(default)]
    pub search_layoutable: Option<bool>,

    /// Indicates whether this object can be searched.
    ///
    /// If `true`, records of this object can be returned by Salesforce search.
    #[serde(default)]
    pub searchable: bool,

    /// Scope metadata supported by this object.
    ///
    /// Supported scopes describe filtering scopes that can be applied when
    /// querying or viewing records, such as "all records" or user-specific scopes.
    #[serde(default)]
    pub supported_scopes: Vec<ScopeInfo>,

    /// Indicates whether Apex triggers are supported for this object.
    ///
    /// If `true`, triggers can be defined for this object.
    #[serde(default)]
    pub triggerable: bool,

    /// Indicates whether deleted records of this object can be undeleted.
    ///
    /// This depends on object support, recycle bin behavior, and user permissions.
    #[serde(default)]
    pub undeletable: bool,

    /// Indicates whether records of this object can be updated.
    ///
    /// This value depends on the object type and the current user's permissions.
    #[serde(default)]
    pub updateable: bool,

    /// URL templates and related REST resource URLs for this object.
    ///
    /// The map can include URLs for resources such as the object endpoint,
    /// describe endpoint, row template, layouts, compact layouts, approval
    /// layouts, list views, quick actions, and default values.
    #[serde(default)]
    pub urls: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NamedLayoutInfo {
    /// Name of this layout.
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScopeInfo {
    /// UI label for this scope..
    label: String,

    /// Name of this scope.
    name: String,
}
