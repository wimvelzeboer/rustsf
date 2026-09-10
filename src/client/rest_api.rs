use crate::rest_api::RestApiBuilder;
use super::Client;

impl Client {

	pub fn rest_api(&self) -> RestApiBuilder {
		RestApiBuilder::new(self)
	}
}