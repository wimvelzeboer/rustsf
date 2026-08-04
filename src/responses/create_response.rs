use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateResponse {
    pub id: String,
    pub success: bool,
}
