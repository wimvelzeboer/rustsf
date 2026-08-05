use crate::client::client::Client;
use crate::errors::Error;
use reqwest::Response;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Default)]
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
    /// let client = Client::new();
    /// let bulk_api = BulkApiV2::new(client);
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
    /// use rustsf::BulkApiV2;
    ///
    /// let params = json!({
    ///     "job_name": "example_job",
    ///     "priority": "high",
    /// });
    ///
    /// let response = BulkApiV2::new(client).create_job(params).await;
    /// match response {
    ///     Ok(res) => println!("Job created successfully: {:?}", res),
    ///     Err(err) => eprintln!("Failed to create job: {:?}", err),
    /// }
    /// ```
    ///
    /// # Notes
    /// - Ensure that the `params` parameter is serializable and adheres to the expected
    ///   format required by the server's API.
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/create_job.htm>
    pub async fn create_job<T: Serialize>(&mut self, params: T) -> Result<Response, Error> {
        let resource_url = format!("{}/jobs/ingest", self.client.base_path()?);
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
    /// let mut api = BulkApiV2::new(client);
    /// let job_id = "12345";
    /// let csv_data = vec![b'h', b'e', b'a', b'd', b'e', b'r', b'\n', b'd', b'a', b't', b'a'];
    /// match api.upload_job_data(job_id, csv_data).await {
    ///     Ok(message) => println!("Success: {}", message),
    ///     Err(e) => eprintln!("Error: {:?}", e),
    /// }
    /// ```
    ///
    /// # Notes
    /// * Ensure that the `client` is properly initialized before invoking this method.
    /// * The endpoint URL is constructed using the base path of the client and appending the job-specific path.
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/upload_job_data.htm>
    pub async fn upload_job_data(&mut self, job_id: &str, csv: Vec<u8>) -> Result<String, Error> {
        let resource_url = format!("{}/jobs/ingest/{}/batches", self.client.base_path()?, job_id);
        let res = self.client.put(resource_url, csv).await?;

        if res.status().is_success() {
            Ok("Created".to_string())
        } else {
            Err(Error::DescribeError(res.json().await?))
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
    /// let mut api = BulkApiV2::new(Client);
    /// match api.get_all_jobs().await {
    ///     Ok(response) => {
    ///         println!("Jobs retrieved successfully: {:?}", response);
    ///     },
    ///     Err(e) => {
    ///         eprintln!("Failed to retrieve jobs: {:?}", e);
    ///     },
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
    pub async fn get_all_jobs(&mut self) -> Result<Response, Error> {
        let resource_url = format!("{}/jobs/ingest/", self.client.base_path()?);
        self.client.get(resource_url, vec![]).await
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
    /// let mut api = BulkApiV2::new(Client);
    /// let job_info = api.get_job_info("job1234").await;
    /// match job_info {
    ///     Ok(response) => println!("Job info retrieved: {:?}", response),
    ///     Err(e) => eprintln!("Failed to retrieve job info: {:?}", e),
    /// }
    /// ```
    ///
    /// # Notes
    /// - Ensure the `client` is properly initialized before calling this function.
    /// - This function uses an asynchronous HTTP GET request.
    ///
    ///  # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/get_job_info.htm>
    pub async fn get_job_info(&mut self, job_id: &str) -> Result<Response, Error> {
        let resource_url = format!("{}/jobs/ingest/{}", self.client.base_path()?, job_id);
        self.client.get(resource_url, vec![]).await
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
    ///let mut api = BulkApiV2::new(Client);
    /// let job_id = "exampleJobId";
    /// let result_set = "successfulResults";
    ///
    /// let response = api.get_job_records(job_id, result_set).await;
    /// match response {
    ///     Ok(res) => {
    ///         // Handle successful response
    ///         println!("Job records: {:?}", res);
    ///     }
    ///     Err(err) => {
    ///         // Handle error
    ///         eprintln!("Error fetching job records: {:?}", err);
    ///     }
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
    ) -> Result<Response, Error> {
        // NOTE: RESULT_SET IS ONE OF successfulResults, failedResults, unprocessedrecords
        let resource_url = format!(
            "{}/jobs/ingest/{}/{}",
            self.client.base_path()?,
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
    /// let mut api = BulkApiV2::new(Client);
    /// let result = api.abort_job("job12345").await;
    /// match result {
    ///     Ok(response) => println!("Job aborted successfully: {:?}", response),
    ///     Err(e) => eprintln!("Failed to abort job: {:?}", e),
    /// }
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/abort_job.htm>
    pub async fn abort_job(&mut self, job_id: &str) -> Result<Response, Error> {
        let resource_url = format!("{}/jobs/ingest/{}", self.client.base_path()?, job_id);
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
    /// let mut api = BulkApiV2::new(Client);
    /// let job_id = "12345";
    /// let params = json!({
    ///     "status": "completed",
    ///     "timestamp": "2023-01-01T12:00:00Z"
    /// });
    /// let response = api.set_upload_state(job_id, params).await?;
    /// println!("Response: {:?}", response);
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/close_job.htm>
    pub async fn set_upload_state<T: Serialize>(
        &mut self,
        job_id: &str,
        params: T,
    ) -> Result<Response, Error> {
        let resource_url = format!("{}/jobs/ingest/{}", self.client.base_path()?, job_id);
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
    /// let mut api = BulkApiV2::new(Client);
    /// let job_id = "12345";
    /// match api.check_job_status(job_id).await {
    ///     Ok(response) => {
    ///         // Handle successful response
    ///         println!("Job status: {:?}", response);
    ///     }
    ///     Err(error) => {
    ///         // Handle error
    ///         eprintln!("Error checking job status: {:?}", error);
    ///     }
    /// }
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/get_job_info.htm>
    pub async fn check_job_status(&mut self, job_id: &str) -> Result<Response, Error> {
        let resource_url = format!("{}/jobs/ingest/{}/", self.client.base_path()?, job_id);
        self.client.get(resource_url, vec![]).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use serde_json::json;

    fn create_test_bulk_api_v2(server_url: &str) -> BulkApiV2 {
        let mut client = Client::new();
        client.set_instance_url(server_url);
        client.set_access_token(
            "test_token".to_string(),
            "9999999999000".to_string(),
            "Bearer".to_string(),
        );
        client.set_version("v60.0");
        BulkApiV2::new(client)
    }

    #[test]
    fn test_new() {
        let client = Client::new();
        let api = BulkApiV2::new(client);
        assert!(api.client.instance_url.is_none());
    }

    #[test]
    fn test_base_path() {
        let mut client = Client::new();
        client.set_instance_url("https://na1.salesforce.com");
        client.set_version("v60.0");
        let api = BulkApiV2::new(client);
        assert_eq!(
            api.client.base_path().unwrap(),
            "https://na1.salesforce.com/services/data/v60.0"
        );
    }

    #[test]
    fn test_base_path_not_logged_in() {
        let client = Client::new();
        let api = BulkApiV2::new(client);
        let result = api.client.base_path();
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::NotLoggedIn => {}
            e => panic!("Expected NotLoggedIn, got {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_create_job() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/services/data/v60.0/jobs/ingest")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "id": "750xx",
                    "operation": "insert",
                    "object": "Account",
                    "createdById": "005xx",
                    "createdDate": "2024-01-01",
                    "systemModstamp": "2024-01-01",
                    "state": "Open",
                    "concurrencyMode": "Parallel",
                    "contentType": "CSV",
                    "apiVersion": 60.0,
                    "contentUrl": "services/data/v60.0/jobs/ingest/750xx/batches",
                    "lineEnding": "LF"
                })
                .to_string(),
            )
            .create_async()
            .await;

        let mut api = create_test_bulk_api_v2(&server.url());
        let mut params = HashMap::new();
        params.insert("operation", "insert");
        params.insert("object", "Account");
        let res = api.create_job(params).await;
        assert!(res.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_upload_job_data_success() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/services/data/v60.0/jobs/ingest/750xx/batches")
            .with_status(201)
            .create_async()
            .await;

        let mut api = create_test_bulk_api_v2(&server.url());
        let csv = b"Name\nTest Account".to_vec();
        let res = api.upload_job_data("750xx", csv).await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "Created");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_upload_job_data_failure() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/services/data/v60.0/jobs/ingest/750xx/batches")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "message": "Invalid CSV",
                    "errorCode": "INVALID_CONTENT"
                })
                .to_string(),
            )
            .create_async()
            .await;

        let mut api = create_test_bulk_api_v2(&server.url());
        let csv = b"bad data".to_vec();
        let res = api.upload_job_data("750xx", csv).await;
        assert!(res.is_err());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_all_jobs() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/services/data/v60.0/jobs/ingest/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({"records": [], "done": true}).to_string())
            .create_async()
            .await;

        let mut api = create_test_bulk_api_v2(&server.url());
        let res = api.get_all_jobs().await;
        assert!(res.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_job_info() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/services/data/v60.0/jobs/ingest/750xx")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({"id": "750xx", "state": "Open"}).to_string())
            .create_async()
            .await;

        let mut api = create_test_bulk_api_v2(&server.url());
        let res = api.get_job_info("750xx").await;
        assert!(res.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_job_records() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock(
                "GET",
                "/services/data/v60.0/jobs/ingest/750xx/successfulResults",
            )
            .with_status(200)
            .with_body("sf__Id,Name\n001xx,Test")
            .create_async()
            .await;

        let mut api = create_test_bulk_api_v2(&server.url());
        let res = api.get_job_records("750xx", "successfulResults").await;
        assert!(res.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_abort_job() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PATCH", "/services/data/v60.0/jobs/ingest/750xx")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({"state": "Aborted"}).to_string())
            .create_async()
            .await;

        let mut api = create_test_bulk_api_v2(&server.url());
        let res = api.abort_job("750xx").await;
        assert!(res.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_set_upload_state() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PATCH", "/services/data/v60.0/jobs/ingest/750xx")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({"state": "UploadComplete"}).to_string())
            .create_async()
            .await;

        let mut api = create_test_bulk_api_v2(&server.url());
        let mut params = HashMap::new();
        params.insert("state", "UploadComplete");
        let res = api.set_upload_state("750xx", params).await;
        assert!(res.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_check_job_status() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/services/data/v60.0/jobs/ingest/750xx/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({"id": "750xx", "state": "JobComplete"}).to_string())
            .create_async()
            .await;

        let mut api = create_test_bulk_api_v2(&server.url());
        let res = api.check_job_status("750xx").await;
        assert!(res.is_ok());
        mock.assert_async().await;
    }
}
