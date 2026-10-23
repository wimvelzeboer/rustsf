use crate::RestApi;
use crate::rest_api::HTTPMethod;
use crate::rest_api::builder::sobjects::SObjectsBuilder;
use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::rest_api::builder::sobjects::updated::sobject_updated_response::SObjectUpdatedResponse;

mod sobject_updated_response;

impl RestApi {

	/// sObject Get Updated
	///
	/// Retrieves the list of individual records that have been updated within the given timespan
	/// for the specified object (`sobject_name`).
	///
	/// # Parameters
	/// - `sobject_name`: The API name of the Salesforce object whose updated records are being queried.
	/// - `start_date`: Starting date/time (Coordinated Universal Time (UTC)—not local— timezone) of the
	///   timespan for which to retrieve the data. The API ignores the seconds portion of the
	///   specified dateTime value (for example, 12:30:15 is interpreted as 12:30:00 UTC).
	///   The date and time must be formatted as described in Valid Date and DateTime Formats.
	///   The date/time value for start must chronologically precede end.
	///   This parameter should be URL-encoded.
	/// - `end_date`: Ending date/time (Coordinated Universal Time (UTC)—not local— timezone) of the
	///   timespan for which to retrieve the data. The API ignores the seconds portion of the
	///   specified dateTime value (for example, 12:35:15 is interpreted as 12:35:00 UTC).
	///   The date and time must be formatted as described in Valid Date and DateTime Formats.
	///   This parameter should be URL-encoded.
	///
	/// # Returns
	/// - `Ok(UpdatedSObjectsResponse)`: A structured response containing the details of the updated objects.
	/// - `Err(Error)`: An error if the request fails or the API response is invalid.
	///
	/// # Notes
	/// - Deleted records are written to a delete log which this resource accesses. A background
	///   process that runs every two hours purges records that have been in an organization's
	///   delete log for more than two hours if the number of records is above a certain limit.
	///   Starting with the oldest records, the process purges delete log entries until the delete
	///   log is back below the limit. This is done to protect Salesforce from performance issues
	///   related to massive delete logs
	/// - Information on deleted records is returned only if the current session user has access to them.
	/// - Results are returned for no more than 15 days previous to the day the call is executed
	///   (or earlier if an administrator has purged the Recycle Bin).
	/// - There is a limit of 600,000 IDs returned from this resource.
	///   If more than 600,000 IDs are found, EXCEEDED_ID_LIMIT is returned.
	///   You can correct the error by choosing start and end dates that are closer together.
	///
	/// # Example
	/// ```rust
	/// use rustsf::{Client, Credentials, RestApi};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     use rustsf::RestApi;
	///     let client= Client::new(Credentials::new()).await?;
	///     // Authentication logic...
	///
	///     let mut api = RestApi::new(client);
	///     let start_date = "2023-01-01";
	///     let end_date = "2023-01-31";
	///     let result = api.sobject_get_deleted("Account", start_date, end_date).await;
	///
	///     match result {
	///         Ok(response) => {
	///             println!("Deleted objects: {:?}", response);
	///         }
	///         Err(e) => {
	///             eprintln!("Error fetching deleted objects: {:?}", e);
	///         }
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # See
	/// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_getupdated.htm>
	pub async fn sobject_get_updated(
		&self,
		sobject_name: &str,
		start_date: &DateTime<Utc>,
		end_date: &DateTime<Utc>,
	) -> Result<SObjectUpdatedResponse> {
		self.build()
			.sobjects()
			.updated(sobject_name, start_date, end_date)
			.send()
			.await
	}
}

impl<'a> SObjectsBuilder<'a> {

	pub fn updated(
		&self,
		sobject_name: &str,
		start_date: &DateTime<Utc>,
		end_date: &DateTime<Utc>,
	) -> SObjectUpdatedBuilder<'_> {
		SObjectUpdatedBuilder::new(self, sobject_name, start_date, end_date)
	}
}

pub struct SObjectUpdatedBuilder<'a> {
	builder: &'a SObjectsBuilder<'a>,
	sobject_name: String,
	start_date: String,
	end_date: String,
}


impl<'a> SObjectUpdatedBuilder<'a> {
	pub(crate) fn new(builder: &'a SObjectsBuilder, sobject_type: &str, start_date: &DateTime<Utc>, end_date: &DateTime<Utc>) -> SObjectUpdatedBuilder<'a>
	{
		Self {
			builder,
			sobject_name: sobject_type.to_string(),
			start_date: start_date.to_string(),
			end_date: end_date.to_string(),
		}
	}

	pub async fn send(
		&self,
	) -> Result<SObjectUpdatedResponse> {
		let url = format!("{}/updated", self.sobject_name);

		let start_formatted = DateTime::parse_from_rfc3339(&self.start_date)
			.map(|dt| dt.with_timezone(&Utc))
			.or_else(|_| self.start_date.parse::<DateTime<Utc>>())
			.map(|dt| dt.format("%Y-%m-%dT%H:%M:%S.000Z").to_string())?;

		let end_formatted = DateTime::parse_from_rfc3339(&self.end_date)
			.map(|dt| dt.with_timezone(&Utc))
			.or_else(|_| self.end_date.parse::<DateTime<Utc>>())
			.map(|dt| dt.format("%Y-%m-%dT%H:%M:%S.000Z").to_string())?;

		let params = vec![
			("start".to_string(), start_formatted),
			("end".to_string(), end_formatted),
		];
		let response = self.builder.send(HTTPMethod::GET, url, &params).await?;
		crate::client::handle_json_response(response).await
	}
}