//! # Salesforce REST API conditional request headers.
//!
//! ## Available headers
//!
//! - ETag
//! - [If-Match](crate::rest_api::builder::RestApiBuilder::if_match_header)
//! - [If-None-Match](crate::rest_api::builder::RestApiBuilder::if_none_match_header)
//! - [If-Modified-Since](crate::rest_api::builder::RestApiBuilder::if_modified_since_header)
//! - [If-Unmodified-Since](crate::rest_api::builder::RestApiBuilder::if_unmodified_since_header)
//!
//! # See
//! https://developer.salesforce.com/docs/platform/api-rest/guide/intro-rest-conditional-requests.html

use chrono::{DateTime, Utc};
use crate::rest_api::builder::RestApiBuilder;

impl<'a> RestApiBuilder<'a> {

	/// The If-Match header is a request header for sObject Rows that includes a list of ETags.
	/// If the ETag of the record that you’re requesting matches an ETag specified as a precondition
	/// in the header, the request is processed.
	///
	/// Otherwise, a 412 Precondition Failed status code is returned, and the request isn’t processed.
	///
	/// # Example
	/// ```text
	/// If-Match: "Jbjuzw7dbhaEG3fd90kJbx6A0ow=-gzip", "U5iWijwWbQD18jeiXwsqxeGpZQk=-gzip"
	/// ```
	///
	/// # See
	/// https://developer.salesforce.com/docs/platform/api-rest/guide/intro-rest-conditional-requests.html#if-match
	pub fn if_match_header(&mut self, etag: &[&str] ) -> &Self {
		let value = etag
			.iter()

			.map(|e| {
				if e.starts_with('"') && e.ends_with('"') {
					e.to_string()
				} else {
					format!("\"{}\"", e)
				}
			})
			.collect::<Vec<_>>()
			.join(", ");

		self.header("If-Match", &value)
	}

	/// The If-None-Match header is a request header for sObject Rows that’s the inverse of If-Match.
	/// If the ETag of the record that you’re requesting matches an ETag specified in the header,
	/// the request isn’t processed.
	/// A 304 Not Modified status code is returned for GET or HEAD requests,
	/// and a 412 Precondition Failed status code is returned for PATCH requests.
	///
	/// This header supports sObject Rows (Account records only) resources.
	///
	/// # Example
	/// ```text
	/// If-None-Match: "Jbjuzw7dbhaEG3fd90kJbx6A0ow=-gzip", "U5iWijwWbQD18jeiXwsqxeGpZQk=-gzip"
	/// ```
	///
	/// # See
	/// https://developer.salesforce.com/docs/platform/api-rest/guide/intro-rest-conditional-requests.html#if-none-match
	pub fn if_none_match_header(&mut self, etag: &[&str] ) -> &Self {
		let value = etag
			.iter()
			.map(|e| {
				if e.starts_with('"') && e.ends_with('"') {
					e.to_string()
				} else {
					format!("\"{}\"", e)
				}
			})
			.collect::<Vec<_>>()
			.join(", ");

		self.header("If-None-Match", &value)
	}


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
	///
	/// # See
	/// https://developer.salesforce.com/docs/platform/api-rest/guide/intro-rest-conditional-requests.html#if-modified-since
	pub fn if_modified_since_header(&mut self, time: &DateTime<Utc>) -> &mut Self {

		let since = time
			.format("%a, %d %b %Y %H:%M:%S %:z")
			.to_string();
		self.header("If-Modified-Since", &since);
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
	///
	/// # See
	/// https://developer.salesforce.com/docs/platform/api-rest/guide/intro-rest-conditional-requests.html#if-unmodified-since
	pub fn if_unmodified_since_header(&mut self, since: &str) -> &mut Self {
		self.header("If-Unmodified-Since", since);
		self
	}
}