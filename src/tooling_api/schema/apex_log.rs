use crate::DefSObject;

use crate as rustsf;

#[DefSObject(sobject_type = "ApexLog")]
pub struct ApexLog {
	/// The name of the application that created the log entry.
	#[serde(rename = "Application")]
	pub application: String,

	/// The execution duration in milliseconds.
	#[serde(rename = "DurationMilliseconds")]
	pub duration: u32,

	/// The location
	#[serde(rename = "Location")]
	pub location: String,

	/// The length of the log file
	#[serde(rename = "LogLength")]
	pub log_length: u32,

	/// The user ID of the user who created the log entry.
	#[serde(rename = "LogUserId")]
	pub log_user_id: String,

	#[serde(rename = "Operation")]
	pub operation: String,

	#[serde(rename = "Request")]
	pub request: String,

	#[serde(rename = "RequestIdentifier")]
	pub request_identifier: String,

	#[serde(rename = "StartTime")]
	pub start_time: String,

	#[serde(rename = "Status")]
	pub status: String,
}

pub const SOBJECT_NAME: &'static str = "ApexLog";

pub const FIELD_NAMES: &'static [&'static str] = &[
	"Id",
	"Application",
	"DurationMilliseconds",
	"Location",
	"LogLength",
	"LogUserId",
	"Operation",
	"Request",
	"RequestIdentifier",
	"StartTime",
	"Status",
];
