use chrono::Utc;
use crate::rest_api::builder::RestApiBuilder;

impl<'a> RestApiBuilder<'a> {

	/// Sets the `If-Modified-Since` header for the request.
	///
	/// The `If-Modified-Since` header is used to make a conditional request,
	/// where the server will only send a response if the requested resource
	/// has been modified since the specified date and time. This can help
	/// optimize bandwidth usage by avoiding sending unchanged data.
	///
	/// # Parameters
	/// - `since`: The modified after date and time, in `EEE, dd MMM yyyy HH:mm:ss z` format
	///   (e.g., `Wed, 21 Oct 2015 07:28:00 GMT`).
	///
	/// # Returns
	/// A mutable reference to `Self`, allowing method chaining.
	///
	/// # Example
	/// ```rust
	/// let mut request = Request::new();
	/// let datetime = Utc::now();
	/// let since = datetime.format("%a, %d %b %Y %H:%M:%S %:z").to_string(); // e.g. "Wed, 11 Sep 2026 20:33:00 GMT"
	/// request.if_modified_since_header(since);
	/// ```
	pub fn if_modified_since_header(&mut self, since: &str) -> &mut Self {
		self.header("If-Modified-Since", since);
		self
	}

	/// Sets the `If-Unmodified-Since` header for the request.
	///
	/// The `If-Unmodified-Since` header is used to make a conditional request,
	/// where the server will only send a response if the requested resource
	/// has been modified since the specified date and time. This can help
	/// optimize bandwidth usage by avoiding sending unchanged data.
	///
	/// # Parameters
	/// - `since`: The modified after date and time, in `EEE, dd MMM yyyy HH:mm:ss z` format
	///   (e.g., `Wed, 21 Oct 2015 07:28:00 GMT`).
	///
	/// # Returns
	/// A mutable reference to `Self`, allowing method chaining.
	///
	/// # Example
	/// ```rust
	/// let mut request = Request::new();
	/// let datetime = Utc::now();
	/// let since = datetime.format("%a, %d %b %Y %H:%M:%S %:z").to_string(); // e.g. "Wed, 11 Sep 2026 20:33:00 GMT"
	/// request.if_unmodified_since_header(since);
	/// ```
	pub fn if_unmodified_since_header(&mut self, since: &str) -> &mut Self {
		self.header("If-Unmodified-Since", since);
		self
	}

}