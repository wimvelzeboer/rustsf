//! # Salesforce Bulk API v1 for high-volume data operations.
//!
//! This module provides a client for interacting with Salesforce's Bulk API (version 1.0).
//! The Bulk API is optimized for processing large sets of data asynchronously.
//!
//! ## Overview
//!
//! The `BulkApi` struct provides methods for:
//! - Creating and managing bulk jobs
//! - Adding batches to jobs with CSV data
//! - Monitoring batch and job status
//! - Retrieving results from completed batches
//! - Closing and aborting jobs
//!
//! ## Example
//!
//! ```rust,ignore
//! use rustsf::{Client, BulkApi, Error};
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct JobParams {
//!     operation: String,
//!     object: String,
//!     content_type: String,
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let mut client = Client::new();
//!     // Perform authentication...
//!     
//!     let mut bulk_api = BulkApi::new(client);
//!     
//!     // Create a job
//!     let params = JobParams {
//!         operation: "insert".to_string(),
//!         object: "Account".to_string(),
//!         content_type: "CSV".to_string(),
//!     };
//!     let job_response = bulk_api.create_job(params).await?;
//!     
//!     // Add batch data
//!     let csv_data = b"Name\nTest Account".to_vec();
//!     let batch_response = bulk_api.add_batch_job("job_id", csv_data).await?;
//!     
//!     // Close the job
//!     bulk_api.close_job("job_id").await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## API Documentation
//!
//! For more information about Salesforce Bulk API, see:
//! <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/>

use crate::client::client::Client;
use crate::errors::Error;
use reqwest::Response;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Default)]
pub struct BulkApi {
    pub(crate) client: Client,
}

impl BulkApi {


    /// Creates a new instance of the `BulkApi` struct.
    ///
    /// # Arguments
    ///
    /// * `client` - A `Client` instance used to perform API requests.
    ///
    /// # Returns
    ///
    /// A new `BulkApi` instance initialized with the provided `Client`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, BulkApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///     let bulk_api = BulkApi::new(client);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(client: Client) -> Self {
        BulkApi { client }
    }

    /// Constructs the base path URL for the asynchronous service endpoint.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - A string representing the constructed base path URL.
    /// * `Err(Error::NotLoggedIn)` - If the `instance_url` is not available, indicating the user is not logged in.
    ///
    /// The method combines the instance URL and version from the client to construct
    /// the endpoint for asynchronous services.
    ///
    /// # Errors
    ///
    /// - Returns an `Error::NotLoggedIn` if the `instance_url` is not set, which
    ///   likely means the client is not authenticated.
    ///
    /// # Example
    /// ```rust,ignore
    ///     let base_path = bulk_api.base_path()?;
    ///     println!("Base path: {}", base_path);
    /// ```
    fn base_path(&self) -> Result<String, Error> {
        let instance_url = self
            .client
            .instance_url
            .as_ref()
            .ok_or(Error::NotLoggedIn)?;

        let version = &self.client.version[1..];
        Ok(format!("{}/services/async/{}", instance_url, version))
    }

    /// Asynchronously creates a new job with the specified parameters.
    ///
    /// This method sends a POST request to the "/job" endpoint using the provided parameters
    /// and authentication headers to create a new job resource.
    ///
    /// # Type Parameters
    /// - `T`: The type of the payload to be serialized and sent as the request body.
    ///         Must implement the `Serialize` trait from `serde`.
    ///
    /// # Parameters
    /// - `params`: The parameters for the job creation, which will be serialized into the request body.
    ///
    /// # Returns
    /// - `Ok(Response)`: If the job creation is successful, returns the server's response wrapped in `Ok`.
    /// - `Err(Error)`: If an issue occurs (e.g., serialization error, network error, or invalid headers),
    ///   returns an `Error` wrapped in `Err`.
    ///
    /// # Errors
    /// - Returns an error if the `base_path` or `get_auth_headers` methods fail (e.g., missing or invalid credentials).
    /// - Returns an error if the HTTP client encounters an issue during the POST request.
    ///
    /// # Example
    /// ```rust
    /// use serde::Serialize;
    /// use rustsf::{Client, BulkApi, Error};
    ///
    /// #[derive(Serialize, Debug)]
    /// struct JobParams {
    ///     name: String,
    ///     priority: u8,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let params = JobParams {
    ///         name: String::from("example-job"),
    ///         priority: 5,
    ///     };
    ///
    ///     let mut bulk_api = BulkApi::new(client);
    ///     let response = bulk_api.create_job(params).await;
    ///     match response {
    ///         Ok(resp) => println!("Job created successfully: {:?}", resp),
    ///         Err(err) => eprintln!("Failed to create job: {}", err),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    /// - Ensure that the API client is properly initialized with valid credentials
    ///   before calling this method.
    /// - The format of the `params` must adhere to the API's expectations for job creation.
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_jobs_create.htm>
    pub async fn create_job<T: Serialize + Debug>(&mut self, params: T) -> Result<Response, Error> {
        let resource_url = format!("{}/job", self.base_path()?);
        let headers = self.get_auth_headers()?;
        self.client.post(resource_url, params, headers).await
    }

    /// Asynchronously adds a batch job to the specified job ID with the provided CSV data.
    ///
    /// # Parameters
    /// - `job_id`: A string slice (`&str`) representing the unique identifier of the job
    ///   to which the batch job will be added.
    /// - `csv`: A vector of bytes (`Vec<u8>`) containing the CSV data that will be sent
    ///   with the batch job.
    ///
    /// # Returns
    /// - `Ok(Response)`: A `Response` object containing the server's response if the
    ///   operation is successful.
    /// - `Err(Error)`: An `Error` object if an error occurs during the operation, such as
    ///   authentication issues, network errors, or invalid input.
    ///
    /// # Errors
    /// This function can return errors under the following conditions:
    /// - If constructing the base path (`self.base_path()`) fails.
    /// - If generating the authentication headers (`self.get_auth_headers()`) fails.
    /// - If the HTTP POST request fails due to network issues or server-side errors.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, BulkApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut bulk_api = BulkApi::new(client);
    ///     let job_id = "12345";
    ///     let csv_data = vec![104, 101, 108, 108, 111]; // Example CSV bytes ("hello")
    ///     match bulk_api.add_batch_job(job_id, csv_data).await {
    ///         Ok(response) => {
    ///             println!("Batch job added successfully: {:?}", response);
    ///         }
    ///         Err(error) => {
    ///             eprintln!("Failed to add batch job: {:?}", error);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Note
    /// - The `Content-Type` header for this request is automatically set to `text/csv`.
    /// - Ensure that the `self.client` is properly initialized and authenticated
    ///   before calling this function.
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_quickstart_add_batch.htm>
    pub async fn add_batch_job(&mut self, job_id: &str, csv: Vec<u8>) -> Result<Response, Error> {
        let resource_url = format!("{}/job/{}/batch", self.base_path()?, job_id);
        let mut headers = self.get_auth_headers()?;
        headers.push(("Content-Type".to_string(), "text/csv".to_string()));
        self.client
            .post_raw_buffer(resource_url, csv, headers)
            .await
    }

    /// Fetches the details of a specific batch associated with a given job.
    ///
    /// This asynchronous function retrieves data for the specified batch, identified by its `batch_id`,
    /// which is associated with the provided `job_id`. The function constructs the appropriate URL
    /// using the job and batch identifiers and sends a GET request to the remote service.
    /// The request includes authentication headers for authorization.
    ///
    /// # Parameters
    /// - `job_id`: A reference to a string slice representing the unique identifier of the job.
    /// - `batch_id`: A reference to a string slice representing the unique identifier of the batch.
    ///
    /// # Returns
    /// - `Ok(Response)`: On success, returns a `Response` object containing the batch details retrieved
    ///   from the remote service.
    /// - `Err(Error)`: If an error occurs during URL construction, header generation, or the HTTP request,
    ///   an error of type `Error` is returned.
    ///
    /// # Errors
    /// This function can return an error in the following cases:
    /// - If the base path cannot be determined (`self.base_path()` fails).
    /// - If authentication headers cannot be generated (`self.get_auth_headers()` fails).
    /// - If the HTTP request to the remote service fails.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, BulkApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut bulk_api = BulkApi::new(client);
    ///     let response = bulk_api.get_batch("123", "456").await;
    ///     match response {
    ///         Ok(data) => println!("Batch details: {:?}", data),
    ///         Err(e) => eprintln!("Error fetching batch: {:?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_quickstart_check_status.htm>
    pub async fn get_batch(&mut self, job_id: &str, batch_id: &str) -> Result<Response, Error> {
        let resource_url = format!("{}/job/{}/batch/{}/", self.base_path()?, job_id, batch_id);
        let headers = self.get_auth_headers()?;
        self.client.get_raw(&resource_url, headers).await
    }

    /// Closes a job by updating its state to "Closed".
    ///
    /// # Parameters
    /// - `job_id`: A string slice that holds the unique identifier of the job to be closed.
    ///
    /// # Returns
    /// An asynchronous `Result` containing:
    /// - `Response`: The server's response if the job closure is successful.
    /// - `Error`: An error indicating what went wrong (e.g., network issues, invalid job ID, or
    /// authentication failure).
    ///
    /// # Errors
    /// This function will return an error in the following cases:
    /// - If `base_path` fails to retrieve the base URL for the request.
    /// - If `get_auth_headers` fails to retrieve the necessary authentication headers.
    /// - If the HTTP POST request to update the job state fails.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, BulkApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut bulk_api = BulkApi::new(client);
    ///     let job_id = "12345";
    ///     match bulk_api.close_job(job_id).await {
    ///         Ok(response) => println!("Job successfully closed: {:?}", response),
    ///         Err(err) => eprintln!("Failed to close job: {:?}", err),
    ///     }
    ///     Ok(())
    ///  }
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_jobs_close.htm>
    pub async fn close_job(&mut self, job_id: &str) -> Result<Response, Error> {
        let resource_url = format!("{}/job/{}", self.base_path()?, job_id);
        let headers = self.get_auth_headers()?;
        let mut params = HashMap::new();
        params.insert("state", "Closed");
        self.client.post(resource_url, params, headers).await
    }

    /// Asynchronously fetches the details of a specific job from the server.
    ///
    /// # Arguments
    ///
    /// * `job_id` - A string slice that holds the unique identifier of the job.
    ///
    /// # Returns
    ///
    /// * `Result<Response, Error>` -
    ///     * On success, returns a `Response` object containing the job details.
    ///     * On failure, returns an `Error` object detailing what went wrong.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * If the `base_path()` method fails to generate a valid base URL.
    /// * If the `get_auth_headers()` method fails to generate the required authentication headers.
    /// * If the HTTP GET request to the server fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustsf::{Client, BulkApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut bulk_api = BulkApi::new(client);
    ///     match bulk_api.get_job_details("12345").await {
    ///         Ok(response) => println!("Job details: {:?}", response),
    ///         Err(e) => eprintln!("Failed to fetch job details: {:?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Remarks
    ///
    /// The function constructs the resource URL by appending the job ID to the base path.
    /// It adds the required authentication headers before sending the GET request using the HTTP client.
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_jobs_get_details.htm>
    pub async fn get_job_details(&mut self, job_id: &str) -> Result<Response, Error> {
        let resource_url = format!("{}/job/{}", self.base_path()?, job_id);
        let headers = self.get_auth_headers()?;
        self.client.get_raw(&resource_url, headers).await
    }

    /// Retrieves the batches associated with a specific job in an asynchronous manner.
    ///
    /// # Arguments
    ///
    /// * `job_id` - A string slice that identifies the job for which the batches are to be retrieved.
    /// * `content_type` - A string slice representing the content type to set for the request headers.
    ///
    /// # Returns
    ///
    /// * `Result<Response, Error>` - Returns a `Response` on success, which contains the raw HTTP response or an `Error` on failure.
    ///
    /// # Errors
    ///
    /// This function will return an `Error` if:
    /// - The `base_path` method fails to generate the base URL.
    /// - The authentication headers cannot be generated.
    /// - The HTTP GET request to the resource URL fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, BulkApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut bulk_api = BulkApi::new(client);
    ///     let job_id = "12345";
    ///     let content_type = "application/json";
    ///
    ///     match bulk_api.get_batches(job_id, content_type).await {
    ///         Ok(response) => {
    ///             println!("Successfully retrieved batches: {:?}", response);
    ///         },
    ///         Err(e) => {
    ///             eprintln!("Error occurred while retrieving batches: {:?}", e);
    ///         },
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_batches_get_info_all.htm>
    pub async fn get_batches(
        &mut self,
        job_id: &str,
        content_type: &str,
    ) -> Result<Response, Error> {
        let resource_url = format!("{}/job/{}/batch", self.base_path()?, job_id);
        let mut headers = self.get_auth_headers()?;
        headers.push(("Content-Type".to_string(), content_type.to_string()));
        self.client.get_raw(&resource_url, headers).await
    }

    /// Asynchronously retrieves the list of results for a specific job and batch.
    ///
    /// # Parameters
    /// - `job_id`: A string slice that holds the unique identifier of the job.
    /// - `batch_id`: A string slice that holds the unique identifier of the batch associated with the job.
    /// - `content_type`: A string slice specifying the desired content type for the response.
    ///
    /// # Returns
    /// - `Ok(Response)`: A successful HTTP response containing the result list for the specified job and batch.
    /// - `Err(Error)`: An error if the operation fails, such as issues with authentication, URL construction, or the HTTP request.
    ///
    /// # Errors
    /// This function may return an error in the following cases:
    /// - If the `base_path` retrieval fails.
    /// - If there's an issue generating the authentication headers.
    /// - If the HTTP `GET` request fails (e.g., network issues or a non-success HTTP status).
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, BulkApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut bulk_api = BulkApi::new(client);
    ///     let job_id = "12345";
    ///     let batch_id = "67890";
    ///     let content_type = "application/json";
    ///    
    ///     match bulk_api.get_result_list(job_id, batch_id, content_type).await {
    ///         Ok(response) => {
    ///             println!("Successfully retrieved results: {:?}", response);
    ///         }
    ///         Err(err) => {
    ///             eprintln!("Error retrieving results: {:?}", err);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    /// - This function uses the `self.base_path()` method to build the URL endpoint.
    /// - It appends necessary authorization headers using `self.get_auth_headers()`.
    /// - The request is dispatched via the `client.get_raw` method.
    ///
    /// # Dependencies
    /// Ensure that the asynchronous runtime (e.g., `tokio` or `async-std`) is properly configured to execute this function.
    /// 
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_batches_get_results.htm>
    pub async fn get_result_list(
        &mut self,
        job_id: &str,
        batch_id: &str,
        content_type: &str,
    ) -> Result<Response, Error> {
        let resource_url = format!(
            "{}/job/{}/batch/{}/result",
            self.base_path()?,
            job_id,
            batch_id
        );
        let mut headers = self.get_auth_headers()?;
        headers.push(("Content-Type".to_string(), content_type.to_string()));
        self.client.get_raw(&resource_url, headers).await
    }

    /// Fetches the result of a specific job batch operation asynchronously.
    ///
    /// This function builds the URL for accessing the result of a job by taking
    /// the job ID, batch ID, and result ID, and it includes the necessary
    /// authentication headers for the request. The result is retrieved
    /// using an HTTP GET request.
    ///
    /// # Arguments
    ///
    /// * `job_id` - A string slice that holds the unique identifier of the job.
    /// * `batch_id` - A string slice that represents the batch ID within the job.
    /// * `result_id` - A string slice that specifies the ID of the desired result.
    ///
    /// # Returns
    ///
    /// * `Ok(Response)` - The raw response containing the result data if the operation is successful.
    /// * `Err(Error)` - If there is an error during URL construction, header generation,
    ///    or while making the HTTP request.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * If the base URL cannot be retrieved through `self.base_path()`.
    /// * If there is an issue generating authentication headers via `self.get_auth_headers()`.
    /// * If the HTTP GET request fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, BulkApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut bulk_api = BulkApi::new(client);
    ///     match bulk_api.get_result("job123", "batch456", "result789").await {
    ///         Ok(response) => println!("Result fetched: {:?}", response),
    ///         Err(error) => println!("Failed to fetch result: {:?}", error),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// Ensure that the `client` instance is properly configured with the
    /// required base path and authentication capabilities before calling this function.
    /// 
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_code_curl_walkthrough_pk_chunking.htm>
    pub async fn get_result(
        &mut self,
        job_id: &str,
        batch_id: &str,
        result_id: &str,
    ) -> Result<Response, Error> {
        let resource_url = format!(
            "{}/job/{}/batch/{}/result/{}",
            self.base_path()?,
            job_id,
            batch_id,
            result_id
        );
        let mut headers = self.get_auth_headers()?;
        headers.push(("Content-Type".to_string(), "application/xml".to_string()));
        self.client.get_raw(&resource_url, headers).await
    }

    /// Aborts a job with the specified `job_id`.
    ///
    /// This function sends a `POST` request to the resource URL corresponding to the job,
    /// updating its state to "Aborted". The request includes authentication headers
    /// and a JSON content type header.
    ///
    /// # Arguments
    ///
    /// * `job_id` - A string slice representing the unique identifier of the job
    ///              to be aborted.
    ///
    /// # Returns
    ///
    /// * `Result<Response, Error>` - On success, returns a `Response` object containing
    ///                                the server's response to the `POST` request.
    ///                                On failure, returns an `Error` describing the problem.
    ///
    /// # Errors
    ///
    /// This function can return an `Error` if:
    /// * The base path for the URL cannot be determined.
    /// * Authentication headers cannot be generated.
    /// * The HTTP client fails to complete the `POST` request.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustsf::{Client, BulkApi, Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut bulk_api = BulkApi::new(client);
    ///     match bulk_api.abort_job("12345").await {
    ///         Ok(response) => println!("Job aborted successfully: {:?}", response),
    ///         Err(error) => eprintln!("Failed to abort job: {:?}", error),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_jobs_abort.htm>
    pub async fn abort_job(&mut self, job_id: &str) -> Result<Response, Error> {
        let resource_url = format!("{}/job/{}", self.base_path()?, job_id);
        let mut headers = self.get_auth_headers()?;
        headers.push(("Content-Type".to_string(), "application/json".to_string()));

        let mut params = HashMap::new();
        params.insert("state", "Aborted");

        self.client.post(resource_url, params, headers).await
    }

    /// Retrieves the authentication headers required for making API requests.
    ///
    /// # Returns
    /// * `Ok(Vec<(String, String)>)` - A vector of header key-value pairs, where each pair represents
    ///   an HTTP header. Currently, the implementation includes the `X-SFDC-Session` header
    ///   with the access token as its value.
    /// * `Err(Error)` - Returns an error if the access token is not available (e.g., when the
    ///   user is not logged in).
    ///
    /// # Errors
    /// * `Error::NotLoggedIn` - Returned if the `access_token` field of the `client` is `None`.
    ///
    /// # Notes
    /// * The `X-SFDC-Session` header is specifically required for API version 1 requests.
    fn get_auth_headers(&self) -> Result<Vec<(String, String)>, Error> {
        let token = self
            .client
            .access_token
            .as_ref()
            .ok_or(Error::NotLoggedIn)?;
        Ok(vec![
            //X-SFDC-Session is needed for API v1 we can just pass it our access token
            (
                "X-SFDC-Session".to_string(),
                token.value.clone(),
            ),
        ])
    }
}

#[cfg(test)]
mod test;
