use serde::Deserialize;
use crate::responses::create_response::CreateResponse;

#[derive(Deserialize, Debug)]
pub struct UpsertResponse {
    create: Option<CreateResponse>,
}
