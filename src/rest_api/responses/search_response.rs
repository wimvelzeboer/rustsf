use serde::Deserialize;
use super::search_record::SearchRecord;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub search_records: Vec<SearchRecord>,
    //    pub metadata: Metadata,
}