use super::salesforce_sobjects_create_response::SalesforceSObjectsCreateResponse;

pub struct SObjectsCreateResponse<'a, T> {
	pub sobjects: &'a mut [T],
	pub response: Vec<SalesforceSObjectsCreateResponse>,
}

impl<'a, T> SObjectsCreateResponse<'a, T> {

	pub fn new(sobjects: &'a mut [T], response: Vec<SalesforceSObjectsCreateResponse>) -> Self {
		Self { sobjects, response }
	}

	pub fn take_response(self) -> Vec<SalesforceSObjectsCreateResponse> {
		self.response
	}
}