//! ActionOverride Metadata of the DescribeSObjectResult
use serde::{Deserialize, Serialize};

/// ActionOverride provides details about an action that replaces the default action pages for an object.
/// For example, an object could be configured to replace the new/create page with a custom page.
/// This type is available in API version 32.0 and later.
///
/// # See
/// <https://developer.salesforce.com/docs/atlas.en-us.api.meta/api/sforce_api_calls_describesobjects_describesobjectresult.htm#ActionOverride>
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionOverride {
    /// Represents the environment to which the action override applies. For example,
    /// a Large value in this field represents the Lightning Experience desktop environment,
    /// and is valid for Lightning pages and Lightning components.
    /// A Small value represents the Salesforce mobile app on a phone or tablet.
    #[serde(default)]
    pub form_factor: String,

    /// Indicates whether the action override is available in the Salesforce mobile app (true) or not (false).
    #[serde(default)]
    pub is_available_in_touch: String,

    /// The name of the action that overrides the default action. For example,
    /// if the new/create page was overridden with a custom action, the name might be “New”.
    #[serde(default)]
    pub name: String,

    /// The ID of the page for the action override.
    #[serde(default)]
    pub page_id: String,

    /// The URL of the item being used for the action override, such as a Visualforce page. Returns as null for Lightning page overrides.
    #[serde(default)]
    pub url: String,
}
