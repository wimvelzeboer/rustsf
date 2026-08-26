use crate::credentials::Credentials;
use anyhow::Result;

pub struct ClientCredentials;

impl ClientCredentials {
    pub fn new(client_id: &str, client_secret: &str, instance_url: &str) -> Result<Credentials> {
        Ok(Credentials {
            access_token: None,
            client_id: Some(client_id.to_string()),
            client_secret: Some(client_secret.to_string()),
            instance_url: Some(instance_url.to_string()),
            login_endpoint: instance_url.to_string(),
            organisation_id: None,
            refresh_token: None,
            user_id: None,
            username: None,
            password: None,
        })
    }
}

