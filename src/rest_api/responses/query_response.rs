use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryResponse<T> {
	pub total_size: i32,
	pub done: bool,
	pub records: Vec<T>,
}
