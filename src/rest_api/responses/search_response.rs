use super::search_record::SearchRecord;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
	pub search_records: Vec<SearchRecord>,
	//    pub metadata: Metadata,
}
