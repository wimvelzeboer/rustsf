use serde::Deserialize;
use crate::responses::search_record::SearchRecord;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub search_records: Vec<SearchRecord>,
    //    pub metadata: Metadata,
}