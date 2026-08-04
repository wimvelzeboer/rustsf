use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AccessToken {
    pub token_type: String,
    pub value: String,
    pub issued_at: String,
}