use serde::{Deserialize, Serialize};

/// Represents a child relationship in a data model, typically used for deserialization of external data.
///
/// The `ChildRelationship` struct holds metadata about a relationship between a parent and child object,
/// including details such as cascading delete behavior, field relationships, and restrictions.
///
/// # Fields
///
/// * `cascade_delete` -
/// * `child_sobject` -
/// * `deprecated_and_hidden` - A boolean value indicating whether the relationship is deprecated
///                              and hidden from standard usage.
///
/// * `field` -
///
/// * `relationship_name` - An optional `String` that represents the developer-provided name for the relationship.
///
/// * `restricted_delete` - A boolean value indicating whether deletions of the child records in the relationship
///                          are restricted.
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildRelationship {
	/// Indicates whether deleting the parent object will cause the child objects to delete as well.
	pub cascade_delete: bool,

	/// The name of the child SObject in the relationship.
	#[serde(rename = "childSObject")]
	pub child_sobject: Option<String>,

	/// Indicates whether the relationship is deprecated and hidden from standard usage.
	pub deprecated_and_hidden: bool,

	/// The name of the field used to reference the relationship.
	pub field: String,

	//    pub junction_id_list_names: [],
	//    pub junction_reference_to: [],
	pub relationship_name: Option<String>,
	pub restricted_delete: bool,
}
