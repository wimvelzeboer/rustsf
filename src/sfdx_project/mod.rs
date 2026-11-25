//! Reads and serializes a subset of Salesforce DX project configuration.
//!
//! The modeled JSON fields are guided by the [Salesforce project schema], but
//! schema defaults are applied to modeled fields when they are omitted. This
//! module does not validate the schema; unmodeled fields are ignored during
//! deserialization and are not preserved when serializing.
//!
//! [Salesforce project schema]: https://github.com/forcedotcom/schemas/blob/main/sfdx-project.schema.json

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::Error;

/// The configuration modeled from an `sfdx-project.json` file.
///
/// Deserialization requires `packageDirectories`, as does the schema.
#[derive(Serialize, Debug, Deserialize)]
pub struct SfdxProject {

	/// Directories containing source to sync with an org.
	#[serde(rename = "packageDirectories")]
	package_directory: Vec<PackageDirectory>,

	/// Namespace identifying a package and its contents.
	#[serde(rename = "namespace")]
	namespace: Option<String>,

	/// Package or package version aliases and their IDs.
	#[serde(rename = "packageAliases", default)]
	package_aliases: HashMap<String, String>,

	/// API version with which the source is compatible.
	#[serde(rename = "sourceApiVersion", default = "default_source_api_version")]
	source_api_version: String,

	/// Salesforce login URL used for authorization.
	#[serde(rename = "sfdcLoginUrl")]
	sfdc_login_url: Option<String>,

	/// Local port used for OAuth authorization.
	#[serde(rename = "oauthLocalPort", default = "default_oauth_local_port")]
	oauth_local_port: u16,
}

fn default_source_api_version() -> String {
	crate::client::DEFAULT_API_VERSION.to_string()
}

fn default_oauth_local_port() -> u16 {
	1717
}

fn default_true() -> bool {
	true
}

impl SfdxProject {
	/// Creates an empty project with the modeled Salesforce schema defaults.
	pub fn new() -> Self {
		Self {
			package_directory: Vec::new(),
			namespace: None,
			package_aliases: HashMap::new(),
			source_api_version: default_source_api_version(),
			sfdc_login_url: None,
			oauth_local_port: default_oauth_local_port(),
		}
	}

	/// Reads a UTF-8 JSON file at `path` and deserializes its project configuration.
	///
	/// Returns an error if the file cannot be read or its contents cannot be
	/// deserialized into this model.
	pub fn from_file(path: &str) -> Result<Self, Error> {
		let json = std::fs::read_to_string(path)?;
		Self::from_json(&json)
	}

	/// Deserializes a project configuration from a JSON string.
	///
	/// Returns an error for invalid JSON or data incompatible with this model.
	/// This does not perform Salesforce schema validation.
	pub fn from_json(json: &str) -> Result<Self, Error> {
		Ok(serde_json::from_str::<SfdxProject>(json)?)
	}

	/// Serializes a project configuration to a JSON string.
	///
	/// Returns an error for JSON failures.
	pub fn to_json(&self) -> Result<String, Error> {
		Ok(serde_json::to_string(self)?)
	}

	/// Writes the serialized struct to the given file.
	///
	pub fn to_file(&self, path: &str) -> Result<(), Error> {
		std::fs::write(path, self.to_json()?)?;
		Ok(())
	}
}

/// A source directory and its optional package configuration.
///
/// `path` is required; omitted defaulted settings use their schema defaults.
#[derive(Serialize, Debug, Deserialize)]
pub struct PackageDirectory {
	/// Path to the source directory.
	path: String,
	/// Whether this is the default package directory.
	#[serde(default = "default_true")]
	default: bool,
	/// Name or alias of the package associated with the directory.
	package: Option<String>,

	/// Human-readable package version name.
	#[serde(rename = "versionName")]
	version_name: Option<String>,

	/// Package version number.
	#[serde(rename = "versionNumber")]
	version_number: Option<String>,

	/// Human-readable package version description.
	#[serde(rename = "versionDescription")]
	version_description: Option<String>,

	/// Path to the scratch org definition file used for the package.
	#[serde(rename = "definitionFile")]
	definition_file: Option<String>,

	/// Packages required by this package.
	dependencies: Option<Vec<Dependency>>,

	/// ID of the ancestor package version.
	#[serde(rename = "ancestorId")]
	ancestor_id: Option<String>,

	/// Apex script run after package installation.
	#[serde(rename = "postInstallScript")]
	post_install_script: Option<String>,

	/// URL shown after package installation.
	#[serde(rename = "postInstallUrl")]
	post_install_url: Option<String>,

	/// URL for package release notes.
	#[serde(rename = "releaseNotesUrl")]
	release_notes_url: Option<String>,

	/// Apex script run when the package is uninstalled.
	#[serde(rename = "uninstallScript")]
	uninstall_script: Option<String>,

	/// Whether to include profile user licenses in the package.
	#[serde(rename = "includeProfileUserLicenses", default)]
	include_profile_user_licenses: bool,

	/// Whether to include transitive dependencies automatically.
	#[serde(rename = "calculateTransitiveDependencies", default)]
	calculate_transitive_dependencies: bool,

	/// Whether to scope profiles to the package.
	#[serde(rename = "scopeProfiles", default)]
	scope_profiles: bool,
}

/// A package dependency and its optional version number.
#[derive(Serialize, Debug, Deserialize)]
pub struct Dependency {
	/// Name, alias, or ID of the required package.
	package: String,

	/// Version of the required package.
	#[serde(rename = "versionNumber")]
	version_number: Option<String>,
}

#[cfg(test)]
mod tests {
	use super::super::client::DEFAULT_API_VERSION;
	use super::*;
	use serde_json::json;

	#[test]
	fn schema_defaults_are_applied() {
		let project = SfdxProject::from_json(r#"{"packageDirectories":[{"path":"force-app"}]}"#).unwrap();
		let value = serde_json::to_value(project).unwrap();
		assert_eq!(value["sourceApiVersion"], DEFAULT_API_VERSION);
		assert_eq!(value["oauthLocalPort"], 1717);
		assert_eq!(value["packageDirectories"][0]["default"], true);
		assert_eq!(value["packageDirectories"][0]["includeProfileUserLicenses"], false);
		assert_eq!(value["packageDirectories"][0]["calculateTransitiveDependencies"], false);
		assert_eq!(value["packageDirectories"][0]["scopeProfiles"], false);
		let new = serde_json::to_value(SfdxProject::new()).unwrap();
		assert_eq!(new["sourceApiVersion"], DEFAULT_API_VERSION);
		assert_eq!(new["oauthLocalPort"], 1717);
	}

	#[test]
	fn explicit_values_are_preserved() {
		let input = json!({"packageDirectories": [{"path": "src", "default": false,
			"includeProfileUserLicenses": true, "calculateTransitiveDependencies": true,
			"scopeProfiles": true}], "sourceApiVersion": "60.0", "oauthLocalPort": 1234,
			"packageAliases": {"example": "0Ho"}});
		let project = SfdxProject::from_json(&input.to_string()).unwrap();
		let value = serde_json::to_value(project).unwrap();
		assert_eq!(value["packageDirectories"][0]["default"], false);
		assert_eq!(value["packageDirectories"][0]["includeProfileUserLicenses"], true);
		assert_eq!(value["packageDirectories"][0]["calculateTransitiveDependencies"], true);
		assert_eq!(value["packageDirectories"][0]["scopeProfiles"], true);
		assert_eq!(value["sourceApiVersion"], "60.0");
		assert_eq!(value["oauthLocalPort"], 1234);
		assert_eq!(value["packageAliases"], input["packageAliases"]);
	}

	#[test]
	fn required_and_invalid_values_are_not_defaulted() {
		assert!(SfdxProject::from_json(r#"{}"#).is_err());
		assert!(SfdxProject::from_json(r#"{"packageDirectories":[{}]}"#).is_err());
		assert!(SfdxProject::from_json(r#"{"packageDirectories":[],"sourceApiVersion":null}"#).is_err());
		assert!(SfdxProject::from_json(r#"{"packageDirectories":[],"oauthLocalPort":"bad"}"#).is_err());
	}
}