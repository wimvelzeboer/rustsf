use crate::Client;
use crate::errors::Error;
use crate::rest_api::responses::error_response::ErrorResponse;
use crate::rest_api::responses::query_response::QueryResponse;
use crate::tooling_api::responses::execute_anonymous_result::ExecuteAnonymousResult;
use crate::tooling_api::responses::create_result::CreateResult;
use reqwest::Response;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use crate::primary_types::SObject;

pub mod responses;
pub mod schema;
pub mod primary_types;

/// Client for the Salesforce Tooling API.
///
/// Provides access to developer tooling functionality including executing
/// anonymous Apex, managing debug logs and trace flags, and querying
/// metadata objects.
#[derive(Default)]
pub struct ToolingApi {
    pub(crate) client: Client,
}

impl ToolingApi {
    pub fn new(client: Client) -> Self {
        ToolingApi { client }
    }

    /// Execute anonymous Apex code.
    ///
    /// Uses `GET /services/data/{version}/tooling/executeAnonymous/` with the
    /// Apex code as a URL query parameter.
    ///
    /// # Note
    ///
    /// The Apex code is sent as a URL query parameter, so very large code blocks
    /// may exceed URL length limits. Debug log output (`System.debug` statements)
    /// is not included in the response — use trace flags and [`get_apex_log_body`](Self::get_apex_log_body)
    /// to retrieve logs.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, ToolingApi, Error, DefSObject};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = ToolingApi::new(client);
    ///     let apex_str = "System.debug('Hello, World!');";
    ///     match api.execute_anonymous(apex_str).await {
    ///         Ok(response) => println!("Query response: {:?}", response),
    ///         Err(error) => println!("Error executing query: {:?}", error),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_tooling.meta/api_tooling/intro_rest_resources.htm>
    pub async fn execute_anonymous(
        &mut self,
        apex_code: &str,
    ) -> Result<ExecuteAnonymousResult, Error> {
        let url = format!("{}/executeAnonymous/", self.base_path()?);
        let params = vec![("anonymousBody".to_string(), apex_code.to_string())];
        let response = self.client.get(url, params, vec![]).await?;

        if response.status().is_success() {
            Ok(response.json::<ExecuteAnonymousResult>().await?)
        } else {
            let errors: Vec<ErrorResponse> = response.json().await?;
            Err(Error::ErrorResponses(errors))
        }
    }

    /// Get the most recent Apex debug log IDs.
    ///
    /// Queries `SELECT Id FROM ApexLog ORDER BY StartTime DESC LIMIT {limit}`.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, ToolingApi, Error, DefSObject};
    /// use rustsf::tooling_api::schema::apex_log::ApexLog;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = ToolingApi::new(client);
    ///     let apex_str = "System.debug('Hello, World!');";
    ///     let response: Vec<ApexLog> = api.get_latest_apex_logs(10).await?;
    ///     println!("Apex logs: {:?}", response),
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_latest_apex_logs(
        &mut self,
        limit: u32,
    ) -> Result<Vec<schema::apex_log::ApexLog>, Error> {
        match self
            .query(&format!(
                "SELECT {} FROM {} ORDER BY StartTime DESC LIMIT {}",
                schema::apex_log::FIELD_NAMES.join(","),
                schema::apex_log::SOBJECT_NAME,
                limit
            ))
            .await
        {
            Ok(query_result) => {
                let mut logs = Vec::new();
                for record in query_result.records {
                    logs.push(serde_json::from_value::<schema::apex_log::ApexLog>(record)?);
                }
                Ok(logs)
            }
            Err(error) => Err(error),
        }
    }

    /// Get the body (raw text) of an Apex debug log.
    ///
    /// Uses `GET /services/data/{version}/sobjects/ApexLog/{log_id}/Body`.
    /// Returns the log content as a string.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, ToolingApi, Error, DefSObject};
    /// use rustsf::tooling_api::schema::apex_log::ApexLog;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = ToolingApi::new(client);
    ///     let apex_str = "System.debug('Hello, World!');";
    ///     let logs: Vec<ApexLog> = api.get_latest_apex_logs(1).await?;
    ///     let log_id = logs.first().unwrap().id().unwrap();
    ///     let body: String = api.get_apex_log_body(&log_id).await?;
    ///     println!("Apex log: {:?}", body),
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_apex_log_body(&mut self, log_id: &str) -> Result<String, Error> {
        let url = format!(
            "{}/sobjects/ApexLog/{}/Body",
            self.client.base_version_path()?,
            log_id
        );
        let response = self.client.get(url, vec![], vec![]).await?;

        if response.status().is_success() {
            Ok(response.text().await.map_err(Error::HttpError)?)
        } else {
            let errors: Vec<ErrorResponse> = response.json().await?;
            Err(Error::ErrorResponses(errors))
        }
    }

    /// Query active DEVELOPER_LOG trace flags for a user.
    ///
    /// Queries `SELECT Id FROM TraceFlag WHERE TracedEntityId = '{user_id}'
    /// AND LogType = 'DEVELOPER_LOG'`.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, ToolingApi, Error, DefSObject};
    /// use rustsf::tooling_api::schema::trace_flag::TraceFlag;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let user_id = client.get_user_id().unwrap().to_string();
    ///     let mut api = ToolingApi::new(client);
    ///     let response: Vec<TraceFlag> = api.get_trace_flags(&user_id).await?;
    ///     println!("Apex trace flags: {:?}", response),
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_trace_flags(&mut self, user_id: &str) -> Result<Vec<schema::trace_flag::TraceFlag>, Error> {
        match self.query(&format!(
            "SELECT {} FROM {} WHERE TracedEntityId = '{}' AND LogType = 'DEVELOPER_LOG'",
            schema::trace_flag::FIELD_NAMES.join(","),
            schema::trace_flag::SOBJECT_NAME,
            user_id
        ))
            .await {
            Ok(query_result) => {
                let mut records = Vec::new();
                for record in query_result.records {
                    records.push(serde_json::from_value::<schema::trace_flag::TraceFlag>(record)?);
                }
                Ok(records)
            },
            Err(error) => Err(error),
        }
    }

    /// Create a new trace flag.
    ///
    /// Uses `POST /services/data/{version}/tooling/sobjects/TraceFlag/`.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, ToolingApi, Error, DefSObject};
    /// use rustsf::tooling_api::schema::trace_flag::TraceFlag;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = ToolingApi::new(client);
    ///     let debug_level = api.get_debug_level("MyDebugLevel").await?;
    ///     let debug_level_id = debug_level.first().unwrap().id.as_ref().unwrap().to_string();
    ///
    ///     let mut trace_flag = TraceFlag::new();
    ///     trace_flag.traced_entity_id = user_id;
    ///     trace_flag.debug_level_id = debug_level_id;
    ///     trace_flag.log_type = "DEVELOPER_LOG".to_string();
    ///     trace_flag.expiration_date = "2026-08-17T23:59:59.000+0000".to_string();
    ///
    ///     let response = api.create_trace_flag(trace_flag).await?;
    ///     println!("Apex trace flag: {:?}", response),
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_trace_flag<T: Serialize + std::fmt::Debug + SObject + Clone>(
        &mut self,
        mut record: T,
    ) -> Result<T, Error> {
        match self.create("TraceFlag", record.clone()).await {
            Ok(response) => {
                let result = serde_json::from_value::<CreateResult>(response)?;
                if result.success {
                    record.set_id(Some(&result.id));
                }
                Ok(record)
            },
            Err(error) => return Err(error),
        }
    }

    /// Update an existing trace flag.
    ///
    /// Uses `PATCH /services/data/{version}/tooling/sobjects/TraceFlag/{id}`.
    pub async fn update_trace_flag<T: Serialize + std::fmt::Debug>(
        &mut self,
        id: &str, // fixme get this from the object itself
        params: T,
    ) -> Result<(), Error> {
        self.update("TraceFlag", id, params).await
    }

    /// Query for a debug level by its DeveloperName.
    ///
    /// Queries `SELECT Id,... FROM DebugLevel WHERE DeveloperName = '{name}'`.
    ///
    /// # Example
    /// ```rust
    /// use rustsf::{Client, ToolingApi, Error, DefSObject};
    /// use rustsf::tooling_api::schema::debug_level::DebugLevel;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = ToolingApi::new(client);
    ///     let response: Vec<DebugLevel> = api.get_debug_level("SFDC_DevConsole").await?;
    ///     println!("Debug Level: {:?}", response),
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_debug_level(&mut self, developer_name: &str) -> Result<Vec<schema::debug_level::DebugLevel>, Error> {
        match self.query(&format!(
            "SELECT {} FROM {} WHERE DeveloperName = '{}'",
            schema::debug_level::FIELD_NAMES.join(","),
            schema::debug_level::SOBJECT_NAME,
            developer_name
        ))
            .await {
            Ok(result) => {
                let mut records = Vec::new();
                for record in result.records {
                    records.push(serde_json::from_value::<schema::debug_level::DebugLevel>(record)?);
                }
                Ok(records)
            },
            Err(error) => Err(error),
        }
    }

    /// Create a new debug level.
    ///
    /// Uses `POST /services/data/{version}/tooling/sobjects/DebugLevel/`.
    ///
    /// # Examples
    /// ```rust
    /// use rustsf::{Client, ToolingApi, Error, DefSObject};
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let client = Client::new();
    ///     // Authentication logic...
    ///
    ///     let mut api = ToolingApi::new(client);
    ///
    ///     let mut params = HashMap::new();
    ///     params.insert("DeveloperName", "SFDataloaderDebug");
    ///     params.insert("MasterLabel", "SFDataloaderDebug");
    ///     params.insert("ApexCode", "FINEST");
    ///     params.insert("Visualforce", "NONE");
    ///     let response = tooling.create_debug_level(params).await?;
    ///     println!("Debug Level: {:?}", response),
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_debug_level<T: Serialize + std::fmt::Debug>(
        &mut self,
        params: T,
    ) -> Result<Value, Error> {
        self.create("DebugLevel", params).await
    }

    // ── Generic Tooling CRUD ────────────────────────────────────────────

    /// Execute a SOQL query against Tooling API objects.
    ///
    /// Queries Tooling API objects such as `ApexClass`, `ApexTrigger`,
    /// `ApexLog`, `TraceFlag`, `DebugLevel`, etc.
    pub async fn query<T: DeserializeOwned>(
        &mut self,
        query: &str,
    ) -> Result<QueryResponse<T>, Error> {
        let query_url = format!("{}/query/", self.base_path()?);
        let params = vec![("q".to_string(), query.to_string())];
        let response = self.client.get(query_url, params, vec![]).await?;
        handle_json_response(response).await
    }

    /// Retrieve a Tooling API SObject by ID.
    pub async fn find_by_id(&mut self, sobject_name: &str, id: &str) -> Result<Value, Error> {
        let url = format!("{}/sobjects/{}/{}", self.base_path()?, sobject_name, id);
        let response = self.client.get(url, vec![], vec![]).await?;
        handle_json_response(response).await
    }

    /// Create a Tooling API SObject.
    pub async fn create<T: Serialize + std::fmt::Debug>(
        &mut self,
        object_name: &str,
        params: T,
    ) -> Result<Value, Error> {
        let url = format!("{}/sobjects/{}", self.base_path()?, object_name);
        let response = self.client.post(url, params, vec![]).await?;
        handle_json_response(response).await
    }

    /// Update a Tooling API SObject.
    pub async fn update<T: Serialize + std::fmt::Debug>(
        &mut self,
        object_name: &str,
        id: &str,
        params: T,
    ) -> Result<(), Error> {
        let url = format!("{}/sobjects/{}/{}", self.base_path()?, object_name, id);
        let response = self.client.patch(url, params).await?;
        handle_empty_response(response).await
    }

    /// Delete a Tooling API SObject.
    pub async fn destroy(&mut self, sobject_name: &str, id: &str) -> Result<(), Error> {
        let url = format!("{}/sobjects/{}/{}", self.base_path()?, sobject_name, id);
        let response = self.client.delete(url).await?;
        handle_empty_response(response).await
    }

    /// Returns the Tooling API base path: `{instance_url}/services/data/{version}/tooling`
    fn base_path(&self) -> Result<String, Error> {
        Ok(format!("{}/tooling", self.client.base_version_path()?))
    }
}

async fn handle_json_response<T: DeserializeOwned>(response: Response) -> Result<T, Error> {
    if response.status().is_success() {
        Ok(response.json().await?)
    } else {
        let errors: Vec<ErrorResponse> = response.json().await?;
        Err(Error::ErrorResponses(errors))
    }
}

async fn handle_empty_response(response: reqwest::Response) -> Result<(), Error> {
    if response.status().is_success() {
        Ok(())
    } else {
        let errors: Vec<ErrorResponse> = response.json().await?;
        Err(Error::ErrorResponses(errors))
    }
}
