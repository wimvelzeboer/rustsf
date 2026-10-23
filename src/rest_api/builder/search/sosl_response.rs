use super::search_record::SearchRecord;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SoslResponse {
	pub response: SearchResponse,
}

impl SoslResponse {
	pub(crate) fn new(response : SearchResponse) -> Self {
		Self { response }
	}

	pub fn response(&self) -> &SearchResponse {
		&self.response
	}

	pub fn take_response(self) -> SearchResponse {
		self.response
	}
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
	pub search_records: Vec<SearchRecord>,
	//    pub metadata: Metadata,
}
