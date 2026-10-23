use std::fmt::Debug;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SObjectsCreateRequest<'a, T: Debug + Serialize> {
	pub all_or_none: bool,
	pub records: &'a [T],
}

impl<'a, T: Debug + Serialize> SObjectsCreateRequest<'a, T> {
	pub fn new(records: &'a [T], all_or_none: bool) -> Self {
		Self { all_or_none, records }
	}
}
