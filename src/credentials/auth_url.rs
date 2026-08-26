use crate::credentials::Credentials;
use regex::Regex;
use anyhow::{Context, Result};

pub struct AuthUrl;

impl AuthUrl {
    pub fn new(url: String) -> Result<Credentials> {
        let re = Regex::new(r"force://([a-zA-Z0-9._-]+):([a-zA-Z0-9._-]*):([a-zA-Z0-9._-]+={0,2})@([a-zA-Z0-9._-]+)").unwrap();
        let caps = re.captures(&url)
            // .with_context(|| "Invalid Authentication URL expected 'force://([a-zA-Z0-9._-]+):([a-zA-Z0-9._-]*):([a-zA-Z0-9._-]+={0,2})@([a-zA-Z0-9._-]+)'".to_string())?;
            .context("Invalid Authentication URL expected 'force://([a-zA-Z0-9._-]+):([a-zA-Z0-9._-]*):([a-zA-Z0-9._-]+={0,2})@([a-zA-Z0-9._-]+)'")?;

        Ok(Credentials {
            access_token: None,
            client_id: Some(caps[1].to_string()),
            client_secret: Some(caps[2].to_string()),
            instance_url: None,
            login_endpoint: caps[4].to_string(),
            organisation_id: None,
            password: None,
            refresh_token: Some(caps[3].to_string()),
            user_id: None,
            username: None,
        })
    }
}
