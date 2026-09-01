use serde::Deserialize;

/// Represents the response structure for describing a global SObject in Salesforce.
///
/// This struct is used to deserialize JSON responses from the Salesforce
/// `Describe Global` API, specifically when querying metadata about SObjects.

/// # See
/// <https://developer.salesforce.com/docs/atlas.en-us.188.0.api.meta/api/sforce_api_calls_describesobjects_describesobjectresult.htm>
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DescribeSObjectResult {
	/// Reserved for future use
	pub activateable: bool,

	/// Indicates that the object can be used in describeCompactLayouts().
	pub compact_layoutable: bool,

	/// Indicates whether the object can be created or not
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
	// pub urls: HashMap<String, String>,
}
