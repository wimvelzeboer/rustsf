use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use super::SalesforceCreateResponse;


pub struct SObjectsCreateResult<T: DeserializeOwned + Serialize> {
	record: T,
	response: SalesforceCreateResponse,
}

impl<T> SObjectsCreateResult<T>
where
	T: DeserializeOwned + Serialize,
{
	pub fn new(record: T, response: SalesforceCreateResponse) -> Self {
		Self { record, response }
	}

	pub fn is_success(&self) -> bool {
		self.response.success
	}

	pub fn get_response(&self) -> &SalesforceCreateResponse {
		&self.response
	}

	pub fn take_stamped_record(self) -> Result<T> {
		let mut data = serde_json::to_value(&self.record)
			.context("Unable to serialize record")?;
		data["Id"] = serde_json::to_value(&self.response.id)
			.context("Unable to serialize Id value from response")?;

		Ok(serde_json::from_value(data)
			.context("Unable to deserialize record")?)
	}

	pub fn take_id(self) -> Option<String> {
		self.response.id
	}

	pub fn take_record(self) -> T {
		self.record
	}

	pub fn take_response(self) -> SalesforceCreateResponse {
		self.response
	}
}
