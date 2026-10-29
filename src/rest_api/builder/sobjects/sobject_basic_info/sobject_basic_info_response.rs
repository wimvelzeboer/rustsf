use crate::rest_api::responses::describe_sobject_result::DescribeSObjectResult;
use crate::rest_api::responses::sobject_record::SObjectRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SObjectBasicInfoResponse {
	pub object_describe: DescribeSObjectResult,
	pub recent_items: Vec<SObjectRecord>,
}

impl SObjectBasicInfoResponse {
	pub fn object_describe(&self) -> &DescribeSObjectResult {
		&self.object_describe
	}

	pub fn recent_items(&self) -> &[SObjectRecord] {
		self.recent_items.as_ref()
	}
}
