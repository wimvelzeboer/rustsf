use super::describe_global_sobject_response::DescribeSObjectResult;
use serde::Deserialize;

/// Represents the response structure for the "Describe Global" API call.
///
/// This struct is used to parse the JSON response from the Salesforce API into a strongly typed structure.
///
/// # Fields
///
/// * `encoding` - The character encoding used for the response. For example, "UTF-8".
/// * `max_batch_size` - The maximum number of records that can be retrieved in a single batch API call.
/// * `sobjects` - A vector containing metadata for all the available objects in the Salesforce organization.
///                 Each object is represented by the `DescribeGlobalSObjectResponse` struct.
///
/// # Attributes
///
/// * `#[derive(Deserialize, Debug)]` - Allows the struct to be deserialized from JSON and supports debug formatting.
/// * `#[serde(rename_all = "camelCase")]` - Ensures the JSON keys use camelCase naming convention during deserialization.
///
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DescribeGlobalResponse {
	pub encoding: String,
	pub max_batch_size: u16,
	pub sobjects: Vec<DescribeSObjectResult>,
}
