//! Salesforce Bulk API v2 for high-volume data operations.
//!
//! This module provides functionality to interact with Salesforce's Bulk API v2, which is
//! designed for loading, updating, upserting, or deleting large numbers of records asynchronously.
//! The Bulk API v2 is optimized for processing large sets of data and provides better performance
//! characteristics compared to the standard REST API for bulk operations.
//!
//! # Features
//!
//! - Create and manage bulk ingest jobs
//! - Upload CSV data for bulk processing
//! - Monitor job status and retrieve results
//! - Abort running jobs
//! - Retrieve successful, failed, and unprocessed records
//!
//! # Usage
//!
//! The typical workflow for using the Bulk API v2 is:
//!
//! 1. Create a job using [`BulkApiV2::create_job`]
//! 2. Upload data using [`BulkApiV2::upload_job_data`]
//! 3. Close the job by setting its state to "UploadComplete" using [`BulkApiV2::set_upload_state`]
//! 4. Monitor job progress using [`BulkApiV2::check_job_status`]
//! 5. Retrieve results using [`BulkApiV2::get_job_records`]
//!
//! # Example
//!
//! ```rust,no_run
//! use rustsf::{Client, Credentials, BulkApiV2};
//! use serde_json::json;
//! use std::collections::HashMap;
//! use anyhow::Result;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Initialize client and authenticate
//!     let mut client= Client::new(Credentials::new()).await?;
//!     // ... authentication logic ...
//!
//!     let mut bulk_api = BulkApiV2::new(client);
//!
//!     // Create a job
//!     let job_params = json!({
//!         "object": "Account",
//!         "operation": "insert"
//!     });
//!     let job_response = bulk_api.create_job(job_params).await?;
//!     let job_id = "750xx"; // Extract from response
//!
//!     // Upload CSV data
//!     let csv_data = b"Name,Industry\nAcme Corp,Technology\nGlobex Inc,Manufacturing".to_vec();
//!     bulk_api.upload_job_data(job_id, csv_data).await?;
//!
//!     // Close the job
//!     let mut close_params = HashMap::new();
//!     close_params.insert("state", "UploadComplete");
//!     bulk_api.set_upload_state(job_id, close_params).await?;
//!
//!     // Check status
//!     let status = bulk_api.check_job_status(job_id).await?;
//!
//!     // Retrieve successful results
//!     let results = bulk_api.get_job_records(job_id, "successfulResults").await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Job States
//!
//! Bulk API v2 jobs progress through several states:
//! - `Open`: Job is created and ready to accept data
//! - `UploadComplete`: Data upload is finished and job is queued for processing
//! - `InProgress`: Job is being processed
//! - `JobComplete`: Job has finished processing
//! - `Failed`: Job has failed
//! - `Aborted`: Job has been aborted
//!
//! # Result Types
//!
//! When retrieving job records, you can specify one of three result types:
//! - `successfulResults`: Records that were successfully processed
//! - `failedResults`: Records that failed processing with error details
//! - `unprocessedrecords`: Records that were not processed
//!
//! # References
//!
//! For more information about Salesforce Bulk API v2, see:
//! - [Bulk API v2 Developer Guide](https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_intro.htm)
//! - [Bulk API v2 Limits](https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_concepts_limits.htm)

use crate::client::client::Client;
use reqwest::Response;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use anyhow::{anyhow, Result};

/// Represents the Bulk API v2 interface for interacting with a Salesforce Bulk API.
///
/// This struct provides the functionality required to make requests to the Bulk API v2 endpoints
/// using an underlying HTTP client.
///
/// ### Example
/// ```
/// use rustsf::{Client, Credentials, BulkApiV2};
/// use anyhow::Result;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let mut client= Client::new(Credentials::new()).await?;
///     // Authentication logic...
///     let bulk_api_v2 = BulkApiV2::new(client);
///     Ok(())
/// }
/// ```
///
/// ### Fields
/// - `client`: A private field wrapping the `Client` instance, which is used to perform HTTP requests
///   to the Salesforce Bulk API v2 endpoints.
///
/// ### Defaults
/// This struct implements the `Default` trait, enabling the creation of a `BulkApiV2` instance with
/// default field values as necessary. You can override these default values after instantiation
/// if needed.
pub struct BulkApiV2 {
    pub(crate) client: Client,
}

impl BulkApiV2 {
    /// Creates a new instance of `BulkApiV2`.
    ///
    /// # Arguments
    ///
    /// * `client` - An instance of the `Client` struct that will be used to initialize the `BulkApiV2` object.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the `BulkApiV2` struct initialized with the provided `client`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, Credentials, BulkApiV2};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut client= Client::new(Credentials::new()).await?;
    ///     // Authentication logic...
    ///
    ///     let mut bulk_api = BulkApiV2::new(client);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(client: Client) -> Self {
        BulkApiV2 { client }
    }

    /// Asynchronously creates a new job by sending a POST request to the "jobs/ingest" endpoint.
    ///
    /// # Generic Parameters
    /// - `T`: A type that implements the `Serialize` trait, representing the parameters
    ///   to be sent in the request body.
    ///
    /// # Arguments
    /// - `params`: A serializable object containing the parameters for the job creation request.
    ///
    /// # Returns
    /// - `Result<Response, Error>`: On success, returns a `Response` object containing
    ///   details of the server's response. On failure, returns an `Error` detailing
    ///   the reason for the failure.
    ///
    /// # Errors
    /// - Returns an error if the client's base path cannot be resolved.
    /// - Returns an error if the HTTP POST request fails.
    ///
    /// # Example
    /// ```rust
    /// use serde_json::json;
    /// use rustsf::{Client, Credentials, BulkApiV2};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut client= Client::new(Credentials::new()).await?;
    ///     // Authentication logic...
    ///     let params = json!({
    ///         "job_name": "example_job",
    ///         "priority": "high",
    ///     });
    ///
    ///     let response = BulkApiV2::new(client).create_job(params).await;
    ///     match response {
    ///         Ok(res) => println!("Job created successfully: {:?}", res),
    ///         Err(err) => eprintln!("Failed to create job: {:?}", err),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    /// - Ensure that the `params` parameter is serializable and adheres to the expected
    ///   format required by the server's API.
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/create_job.htm>
    pub async fn create_job<T: Serialize + Debug>(&mut self, params: T) -> Result<Response> {
        let resource_url = format!("{}/jobs/ingest", self.client.base_version_path()?);
        self.client.post(resource_url, params, vec![]).await
    }

    /// Asynchronously uploads job data in CSV format to a specified job endpoint.
    ///
    /// # Arguments
    /// * `job_id` - A string slice representing the unique identifier of the job.
    /// * `csv` - A vector of bytes representing the CSV data to be uploaded.
    ///
    /// # Returns
    /// * `Ok(String)` - A success message ("Created") if the upload is successful.
    /// * `Err(Error)` - An error if the upload fails. The error may include a description
    /// of the failure derived from the response.
    ///
    /// # Errors
    /// This function returns an `Error` in the following cases:
    /// * If constructing the resource URL fails.
    /// * If the HTTP PUT request fails or if the server responds with a non-success status code.
    /// * If parsing the response JSON to extract the error description fails.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, Credentials, BulkApiV2};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut client= Client::new(Credentials::new()).await?;
    ///     // Authentication logic...
    ///
    ///     let mut bulk_api = BulkApiV2::new(client);
    ///     let job_id = "12345";
    ///     let csv_data = vec![b'h', b'e', b'a', b'd', b'e', b'r', b'\n', b'd', b'a', b't', b'a'];
    ///     match bulk_api.upload_job_data(job_id, csv_data).await {
    ///         Ok(message) => println!("Success: {}", message),
    ///         Err(e) => eprintln!("Error: {:?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    /// * Ensure that the `client` is properly initialized before invoking this method.
    /// * The endpoint URL is constructed using the base path of the client and appending the job-specific path.
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/upload_job_data.htm>
    pub async fn upload_job_data(&mut self, job_id: &str, csv: Vec<u8>) -> Result<String> {
        let resource_url = format!("{}/jobs/ingest/{}/batches", self.client.base_version_path()?, job_id);
        let res = self.client.put(resource_url, csv).await?;

        if res.status().is_success() {
            Ok("Created".to_string())
        } else {
            Err(anyhow!("Describe error: {:?}", res.text().await?))
        }
    }


    /// Asynchronously retrieves a list of all jobs from the ingest endpoint.
    ///
    /// This function builds the URL for the "jobs/ingest" endpoint by appending
    /// the path to the client's base URL. It then performs an HTTP GET request
    /// to the constructed URL.
    ///
    /// # Returns
    ///
    /// * `Ok(Response)` - If the request was successful, returns the response
    ///   containing the list of jobs.
    /// * `Err(Error)` - If an error occurred during the request, returns the encountered error.
    ///
    /// # Errors
    ///
    /// This function can return an error in the following cases:
    /// - Failure to resolve the client's base path.
    /// - Failure or timeout during the HTTP GET request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustsf::{Client, Credentials, BulkApiV2};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut client= Client::new(Credentials::new()).await?;
    ///     // Authentication logic...
    ///
    ///     let mut bulk_api = BulkApiV2::new(client);
    ///
    ///     match bulk_api.get_all_jobs().await {
    ///         Ok(response) => {
    ///             println!("Jobs retrieved successfully: {:?}", response);
    ///         },
    ///         Err(e) => {
    ///             eprintln!("Failed to retrieve jobs: {:?}", e);
    ///         },
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Note
    /// The caller must ensure that the client is properly configured
    /// with the required base path and authentication (if applicable)
    /// before invoking this function.
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/get_all_jobs.htm>
    pub async fn get_all_jobs(&mut self) -> Result<Response> {
        let resource_url = format!("{}/jobs/ingest/", self.client.base_version_path()?);
        self.client.get(resource_url, vec![], vec![]).await
    }

    /// Retrieves job information for a specified job ID asynchronously.
    ///
    /// # Parameters
    /// - `job_id`: A string slice that holds the identifier of the job whose information is to be retrieved.
    ///
    /// # Returns
    /// - `Result<Response, Error>`: On success, returns a `Response` object containing the job details.
    ///   On failure, returns an `Error` indicating what went wrong during the API call.
    ///
    /// # Errors
    /// - Returns an `Error` if constructing the resource URL fails.
    /// - Returns an `Error` if the `get` request to the client fails.
    ///
    /// # Examples
    /// ```
    /// use rustsf::{Client, Credentials, BulkApiV2};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut client= Client::new(Credentials::new()).await?;
    ///     // Authentication logic...
    ///
    ///     let mut bulk_api = BulkApiV2::new(client);
    ///     let job_info = bulk_api.get_job_info("job1234").await;
    ///     match job_info {
    ///         Ok(response) => println!("Job info retrieved: {:?}", response),
    ///         Err(e) => eprintln!("Failed to retrieve job info: {:?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    /// - Ensure the `client` is properly initialized before calling this function.
    /// - This function uses an asynchronous HTTP GET request.
    ///
    ///  # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/get_job_info.htm>
    pub async fn get_job_info(&mut self, job_id: &str) -> Result<Response> {
        let resource_url = format!("{}/jobs/ingest/{}", self.client.base_version_path()?, job_id);
        self.client.get(resource_url, vec![], vec![]).await
    }

    /// Retrieves job records based on the specified job ID and result set type.
    ///
    /// This asynchronous function fetches job-related data from the ingest jobs endpoint.
    /// The `result_set` parameter specifies the type of records to retrieve, which must be
    /// one of `successfulResults`, `failedResults`, or `unprocessedrecords`.
    ///
    /// # Parameters
    ///
    /// * `job_id` - A reference to a string representing the unique identifier of the job.
    /// * `result_set` - A reference to a string specifying the type of results to fetch.
    ///   Must be one of the following values:
    ///   - `successfulResults`
    ///   - `failedResults`
    ///   - `unprocessedrecords`
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// * `Response` - If the operation is successful, the raw response of the requested job records.
    /// * `Error` - If an error occurs during the request.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The base path of the client cannot be resolved.
    /// * The `GET` request to the resource endpoint fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustsf::{Client, Credentials, BulkApiV2};
    /// use anyhow::Result;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut client= Client::new(Credentials::new()).await?;
    ///     // Authentication logic...
    /// 
    ///     let mut api = BulkApiV2::new(client);
    ///     let job_id = "exampleJobId";
    ///     let result_set = "successfulResults";
    ///     
    ///     let response = api.get_job_records(job_id, result_set).await;
    ///     match response {
    ///         Ok(res) => {
    ///             // Handle successful response
    ///             println!("Job records: {:?}", res);
    ///         }
    ///         Err(err) => {
    ///             // Handle error
    ///             eprintln!("Error fetching job records: {:?}", err);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// Ensure that the `result_set` parameter is one of the allowed values; passing an invalid
    /// value will result in an error from the server.
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/get_job_successful_results.htm>
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/get_job_failed_results.htm>
    pub async fn get_job_records(
        &mut self,
        job_id: &str,
        result_set: &str,
    ) -> Result<Response> {
        // NOTE: RESULT_SET IS ONE OF successfulResults, failedResults, unprocessedrecords
        let resource_url = format!(
            "{}/jobs/ingest/{}/{}",
            self.client.base_version_path()?,
            job_id,
            result_set
        );
        self.client.get_raw(&resource_url, vec![]).await
    }

    /// Aborts an ongoing job by updating its state to "Aborted".
    ///
    /// # Parameters
    /// - `job_id`: A string slice that holds the unique identifier of the job to be aborted.
    ///
    /// # Returns
    /// - `Result<Response, Error>`: Returns a `Response` object if the operation is successful,
    ///   or an `Error` if the operation fails.
    ///
    /// # Errors
    /// - Returns an error if the client's `base_path` cannot be resolved.
    /// - Returns an error if the PATCH request to update the job's state fails.
    ///
    /// # Examples
    /// ```rust
    /// use rustsf::{Client, Credentials, BulkApiV2};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut client= Client::new(Credentials::new()).await?;
    ///     // Authentication logic...
    /// 
    ///     let mut api = BulkApiV2::new(client);
    ///     let result = api.abort_job("job12345").await;
    ///     match result {
    ///         Ok(response) => println!("Job aborted successfully: {:?}", response),
    ///         Err(e) => eprintln!("Failed to abort job: {:?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/abort_job.htm>
    pub async fn abort_job(&mut self, job_id: &str) -> Result<Response> {
        let resource_url = format!("{}/jobs/ingest/{}", self.client.base_version_path()?, job_id);
        let mut params = HashMap::new();
        params.insert("state", "Aborted");
        self.client.patch(resource_url, params).await
    }

    /// Asynchronously sets the upload state for a given job.
    ///
    /// This function sends a PATCH request to update the state of an upload job
    /// identified by the provided `job_id`. The `params` parameter allows you to
    /// specify the necessary details for the update using any type that implements
    /// the `Serialize` trait.
    ///
    /// # Type Parameters
    /// - `T`: A type that implements the `Serialize` trait, representing the
    ///   parameters to send in the PATCH request.
    ///
    /// # Parameters
    /// - `&mut self`: A mutable reference to the current instance of the client.
    /// - `job_id`: A string slice that identifies the specific upload job to update.
    /// - `params`: Serialize-able data containing the parameters for the state change.
    ///
    /// # Returns
    /// - [`Result<Response, Error>`]: If successful, returns an instance of `Response`.
    ///   On failure, returns an `Error`.
    ///
    /// # Errors
    /// Returns an `Error` if:
    /// - Constructing the resource URL fails.
    /// - Sending the PATCH request fails or yields an error response.
    ///
    /// # Example
    /// ```
    /// use rustsf::{Client, Credentials, BulkApiV2};
    /// use serde_json::json;
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut client= Client::new(Credentials::new()).await?;
    ///     // Authentication logic...
    /// 
    ///     let mut api = BulkApiV2::new(client);
    ///     let job_id = "12345";
    ///     let params = json!({
    ///         "status": "completed",
    ///         "timestamp": "2023-01-01T12:00:00Z"
    ///     });
    ///     match api.set_upload_state(job_id, params).await {
    ///         Ok(response) => println!("Upload state set successfully: {:?}", response),
    ///         Err(e) => eprintln!("Failed to set upload state: {:?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/close_job.htm>
    pub async fn set_upload_state<T: Serialize + Debug>(
        &mut self,
        job_id: &str,
        params: T,
    ) -> Result<Response> {
        let resource_url = format!("{}/jobs/ingest/{}", self.client.base_version_path()?, job_id);
        self.client.patch(resource_url, params).await
    }

    /// Asynchronously checks the status of a job with the given job ID.
    ///
    /// # Arguments
    ///
    /// * `job_id` - A string slice that holds the identifier of the job whose status is to be checked.
    ///
    /// # Returns
    ///
    /// This function returns a `Result<Response, Error>` where:
    /// * `Ok(Response)` contains the response from the server if the request is successful.
    /// * `Err(Error)` contains the error information if the request fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The `base_path` of the client cannot be resolved.
    /// * The HTTP GET request to the job status endpoint fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, Credentials, BulkApiV2};
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut client= Client::new(Credentials::new()).await?;
    ///     // Authentication logic...
    /// 
    ///     let mut api = BulkApiV2::new(client);
    ///     let job_id = "12345";
    ///     match api.check_job_status(job_id).await {
    ///         Ok(response) => {
    ///             // Handle successful response
    ///             println!("Job status: {:?}", response);
    ///         }
    ///         Err(error) => {
    ///             // Handle error
    ///             eprintln!("Error checking job status: {:?}", error);
    ///         }
    ///     }
    ///     Ok(())
    /// } 
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/get_job_info.htm>
    pub async fn check_job_status(&mut self, job_id: &str) -> Result<Response> {
        let resource_url = format!("{}/jobs/ingest/{}/", self.client.base_version_path()?, job_id);
        self.client.get(resource_url, vec![], vec![]).await
    }
}

#[cfg(test)]
mod test;