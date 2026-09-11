use crate::rest_api::builder::RestApiBuilder;

impl<'a> RestApiBuilder<'a> {

	/// Specifies options used in a query, such as the query results batch size.
	/// Use this request header with the Query resource.
	///
	/// This method modifies the headers of the current object by inserting a new
	/// header with the key `"Sforce-Query-Options"` and a value in the format
	/// `"batchSize={}"`. The specified `batch_size` is used to construct the value.
	///
	/// # Arguments
	///
	/// * `batch_size` -  The number of records returned for a query request.
	/// Child objects count toward the number of records for the batch size.
	/// For example, in relationship queries, multiple child objects are returned per parent row returned.
	///
	/// # Returns
	///
	/// A mutable reference to the current object, allowing for method chaining.
	///
	/// # Note
	/// The default is 2,000; the minimum is 200, and the maximum is 2,000. There is no guarantee
	/// that the requested batch size is the actual batch size.
	///
	/// # Example
	///
	/// ```rust
	/// let mut client = MyClient::new();
	/// client.query_options_header(500);
	/// ```
	///
	/// In the above example, the header `"Sforce-Query-Options"` will be set
	/// with the value `"batchSize=500"`.
	///
	/// # See
	/// https://developer.salesforce.com/docs/platform/api-rest/guide/headers-queryoptions.html
	pub fn query_options_header(&mut self, batch_size: u16) -> &mut Self {

		let batch_size = match batch_size {
			x if x < 200 => 200,
			x if x > 2000 => 2000,
			x => x
		};

		let value = format!("batchSize={}", batch_size);
		self.headers.insert("Sforce-Query-Options".to_string(), value);
		self
	}
}