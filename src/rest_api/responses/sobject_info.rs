use super::describe_sobject_result::DescribeSObjectResult;
use super::sobject_record::SObjectRecord;
use serde::{Deserialize, Serialize};

///
/// # See
/// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_basic_info_get.htm>
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SObjectInfo {
	pub object_describe: DescribeSObjectResult,
	pub recent_items: Vec<SObjectRecord>,
}
