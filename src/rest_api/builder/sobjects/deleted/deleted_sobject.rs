use crate::primary_types::Id18;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedSObject {
	/// e.g. "a00D0000008pQRAIA2"
	pub id: Id18,

	/// e.g. "2013-05-07T22:07:19.000+0000"
	pub deleted_date: String,
}
