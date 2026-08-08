use super::deleted_sobject::DeletedSObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedSObjectsResponse {
    pub deleted_records: Vec<DeletedSObject>,

    /// e.g. "2013-05-03T15:57:00.000+0000"
    pub earliest_date_available: String,

    /// e.g. "2013-05-03T15:57:00.000+0000"
    pub latest_date_covered: String,
}
