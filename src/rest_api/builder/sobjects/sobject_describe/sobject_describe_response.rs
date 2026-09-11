use serde::{Deserialize, Serialize};
use crate::rest_api::responses::describe_sobject_result::DescribeSObjectResult;
use crate::rest_api::responses::sobject_record::SObjectRecord;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SObjectDescribeResponse {
	pub object_describe: DescribeSObjectResult,
	pub recent_items: Vec<SObjectRecord>,
}

impl  SObjectDescribeResponse {

	pub fn object_describe(&self) -> &DescribeSObjectResult {
		&self.object_describe
	}

	pub fn recent_items(&self) -> &[SObjectRecord] {
		self.recent_items.as_ref()
	}
}