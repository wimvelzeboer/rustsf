use rustsf::{Client, RestApi, DefSObject};
use std::env;
use anyhow::Result;

pub async fn get_rest_api_client() -> Result<RestApi> {
    let mut client = Client::new();
    client.login_with_sfdx_auth_url(&env::var("SCRATCH_AUTH_URL").expect("SCRATCH_AUTH_URL not set")).await?;
    Ok(RestApi::new(client))
}

#[DefSObject(sobject_type = "Account", fields="name,owner")]
pub struct Account {

}

impl Account {
    pub fn set_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}