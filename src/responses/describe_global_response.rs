use serde::Deserialize;
use crate::responses::describe_global_sobject_response::DescribeGlobalSObjectResponse;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DescribeGlobalResponse {
    pub encoding: String,
    pub max_batch_size: u16,
    pub sobjects: Vec<DescribeGlobalSObjectResponse>,
}