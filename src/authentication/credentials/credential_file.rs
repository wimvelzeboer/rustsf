use crate::credentials::Credentials;
use anyhow::{Context, Result};

pub struct CredentialFile;

impl CredentialFile {
	pub fn new(file: &str) -> Result<Credentials> {
		let file_contents =
			std::fs::read_to_string(file).with_context(|| format!("Failed to open credentials file: {}", file))?;
		Ok(serde_json::from_str(&file_contents)
			.with_context(|| format!("Failed parsing the credentials file: {}", file))?)
	}
}
