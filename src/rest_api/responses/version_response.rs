//! Version response structures for Salesforce REST API.
//!
//! This module contains the data structures used to deserialize version information
//! responses from the Salesforce REST API. The version endpoint returns metadata
//! about available API versions, including version numbers, labels, and URLs to
//! access version-specific resources.
//!
//! # See
//! <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/dome_versions.htm>

use serde::Deserialize;

/// A struct representing the response for a version endpoint, typically used
/// to provide information about a specific version of an application or service.
///
/// This struct is deserialized from a JSON response with camelCase keys.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VersionResponse {
    
    /// A human-readable label for the version
    pub label: String,
    
    /// The URL associated with this version, which could point to documentation, a changelog, or a download link
    pub url: String,
    
    /// The actual version string (e.g., "67.0").
    pub version: String,
}