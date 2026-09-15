pub struct RestResponse {
	status: reqwest::StatusCode,

}

impl RestResponse {

	pub fn new(response: reqwest::Response) -> Self {
		Self {
			status: response.status(),
		}
	}
}