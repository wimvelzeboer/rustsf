//! Collects Salesforce metadata members and renders a package manifest.

use std::collections::HashMap;
use std::path::Path;

/// A collection of metadata members grouped by metadata type.
///
/// Members retain their insertion order within each type. Duplicate members
/// are retained, and the order of metadata types is unspecified.
pub struct Manifest {
	elements: HashMap<String, Vec<String>>,
}

impl Manifest {
	/// Creates an empty manifest.
	pub fn new() -> Self {
		Self {
			elements: HashMap::new(),
		}
	}

	/// Appends a member to the metadata type identified by `key`.
	pub fn add_element(&mut self, key: String, value: String) {
		self.elements.entry(key).or_insert(Vec::new()).push(value);
	}

	/// Appends the filename's stem as a member of the metadata type `key`.
	///
	/// The final extension is removed. If the path has no file stem, nothing
	/// is added; non-Unicode characters in the stem are replaced lossily.
	pub fn add_filename(&mut self, key: &str, filename: &str) {
		if let Some(stem) = Path::new(&filename).file_stem() {
			self.elements
				.entry(key.to_string())
				.or_insert(Vec::new())
				.push(stem.to_string_lossy().into());
		};
	}

	/// Returns the members for `key`, or `None` if the type is absent.
	pub fn get_elements(&self, key: &str) -> Option<&Vec<String>> {
		self.elements.get(key)
	}

	/// Returns whether the manifest contains no metadata types.
	pub fn is_empty(&self) -> bool {
		self.elements.is_empty()
	}

	/// Renders the manifest as Salesforce package XML with the given API version.
	///
	/// Metadata types appear in an unspecified order. Member names, type names,
	/// and the API version are inserted without XML escaping.
	pub fn to_xml(&self, api_version: &str) -> String {
		let header = format!("{}\n{}",
			r#"<?xml version="1.0" encoding="UTF-8"?>"#,
			r#"<Package xmlns="http://soap.sforce.com/2006/04/metadata">"#
		);
		let footer = format!("  <version>{}</version>\n</Package>", api_version);

		let mut metadata_type = "".to_string();
		for (metadata, objects) in &self.elements {
			let members = objects
				.iter()
				.map(|object| format!("    <members>{}</members>", object))
				.collect::<Vec<String>>()
				.join("\n");

			metadata_type
				.push_str(format!("  <types>\n{}\n    <name>{}</name>\n  </types>\n", members, metadata).as_str());
		}
		format!("{}\n{}\n{}", header, metadata_type, footer)
	}
}
