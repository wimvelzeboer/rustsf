use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SObjectCreateRequest<T: Serialize> {

    pub all_or_none: bool,
    pub records: Vec<T>,
}

impl<T: Serialize> SObjectCreateRequest<T> {
    pub fn new(records: Vec<T>, all_or_none: bool) -> Self {
        Self {
            all_or_none,
            records,
        }
    }   
}