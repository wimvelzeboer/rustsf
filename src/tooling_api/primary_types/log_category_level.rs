use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone, Eq, PartialEq, Hash, Copy, PartialOrd, Ord)]
pub enum LogCategoryLevel {
	#[default]
	#[serde(rename = "NONE")]
	None,
	#[serde(rename = "ERROR")]
	Error,
	#[serde(rename = "WARN")]
	Warning,
	#[serde(rename = "INFO")]
	Info,
	#[serde(rename = "DEBUG")]
	Debug,
	#[serde(rename = "FINE")]
	Fine,
	#[serde(rename = "FINER")]
	Finer,
	#[serde(rename = "FINEST")]
	Finest,
}
