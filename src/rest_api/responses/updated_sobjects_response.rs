use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedSObjectsResponse {
	/// e.g. [ "a00D0000008pQR5IAM", "a00D0000008pQRGIA2", "a00D0000008pQRFIA2"]
	pub ids: Vec<String>,

	/// e.g. "2013-05-03T15:57:00.000+0000"
	pub latest_date_covered: String,
}
