//! Salesforce REST API
//!
//! This crate provides a Rust client for interacting with the Salesforce REST API. It allows you to
//! perform CRUD operations on Salesforce objects, query data, and more.
//!
//! ## Supported Endpoint
//!
//! - [**/services/data/vXX.X/sobjects**](crate::rest_api::sobjects), CRUD operations on a single Salesforce object
//! - [**/services/data/vXX.X/composite/sobjects**](crate::rest_api::composite), CRUD operations for up to 200 Salesforce objects
//! - [**/services/data/vXX.X/search**](crate::rest_api::search), SOSL search operations on Salesforce objects
//! - [**/services/data/vXX.X/services**](crate::rest_api::services), system information and metadata
//! - [**/services/data/vXX.X/query**](crate::rest_api::query), retrieving up to 2,000 records
//! - [**/services/data/vXX.X/userPassword**](crate::rest_api::user_password), user password client
//!
//! ## Reference
//! https://developer.salesforce.com/docs/platform/api-rest/guide/resources-list.html
//!
//! - [Headers](crate::rest_api::builder::headers)
//!     - [Assignment Rule Header](crate::rest_api::builder::headers::assignment_rule_header)
//!     - [Call Options Header](crate::rest_api::builder::headers::call_options_header)
//!     - Compression Headers
//!     - [Conditional Request Headers](crate::rest_api::builder::headers::conditional_request_headers)
//!     - [Duplicate Rule Header](crate::rest_api::builder::headers::duplicate_rule_header)
//!     - [Limit Info Header](crate::rest_api::builder::headers::limit_info_header)
//!     - [MRU Header](crate::rest_api::builder::headers::mru_header)
//!     - [Package Version Header](crate::rest_api::builder::headers::package_version_header)
//!     - [Query Options Header](crate::rest_api::builder::headers::query_options_header)
//!     - Warning Header
//!
//! - [Versions](crate::rest_api::RestApi::versions)
//! - Resources by Version
//! - Limits
//! - [Describe Global](crate::rest_api::builder::sobjects::SObjectsBuilder::describe_global)
//! - sObject Basic Information
//!     - [Get Object Metadata Using sObject Basic Information](crate::rest_api::builder::sobjects::SObjectsBuilder::basic_info)
//!     - [Create Records Using sObject Basic Information](crate::rest_api::builder::sobjects::SObjectsBuilder::create)
//! - [sObject Desribe](crate::rest_api::builder::sobjects::SObjectsBuilder::describe)
//! - [sObject Get Deleted](crate::rest_api::builder::sobjects::SObjectsBuilder::deleted)
//! - [sObject Get Updated](crate::rest_api::builder::sobjects::SObjectsBuilder::updated)
//! - sObject Named Layouts
//! - sObject Rows
//!     - [Get Records Using sObject Rows](crate::rest_api::builder::sobjects::SObjectsBuilder::get_by_id)
//!     - [Update Records Using sObject Rows](crate::rest_api::builder::sobjects::SObjectsBuilder::update)
//!     - [Delete Records Using sObject Rows](crate::rest_api::builder::sobjects::SObjectsBuilder::delete)
//! - sObject Rows by External ID
//!     - [Get Records Using sObject Rows by External ID](crate::rest_api::builder::sobjects::SObjectsBuilder::get_by_ext_id)
//!     - Create Records Using sObject Rows by External ID
//!     - [Upsert Records Using sObject Rows by External ID](crate::rest_api::builder::sobjects::SObjectsBuilder::upsert_by_ext_id)
//!     - Delete Records Using sObject Rows by External ID
//!     - Return Headers Using sObject Rows by External ID
//! - sObject Blob Get
//! - sObject ApprovalLayouts
//!     - Get Approval Layouts
//!     - Return Headers for Approval Layouts
//! - sObject Single Approval Process
//!     - Get a Layout for a Single Approval Process on a Specified Objec
//!     - Return Headers for a Single Approval Process on a Specified Object
//! - sObject CompactLayouts
//!     - Get Compact Layouts Using sObject CompactLayouts
//!     - Return Headers Using sObject CompactLayouts
//! - sObject Layouts
//!     - Get Layouts and Descriptions for a Specified Object
//!     - Return Layout Headers for a Specified Object
//! - sObject Layouts for an Object With Multiple Record Types
//!     - Get Layouts and Descriptions for an Object With Multiple Record Types
//!     - Return Layout Headers for an Object With Multiple Record Types
//! - sObject Global Publisher Layouts
//!     - Get Global Publisher Layouts and Descriptions
//!     - Return Headers for All Global Publisher Layouts
//! - sObject PlatformAction
//!     - sObject Quick Actions
//!     - Return Headers Using sObject Quick Actions
//! - sObject Specific Quick Actions
//!     - Get Specific sObject Quick Actions
//!     - Create Records Using Specific sObject Quick Actions
//!     - Return Headers Using Specific sObject Quick Actions
//! - sObject Quick Action Details
//!     - Get sObject Quick Action Details
//!     - Return Headers Using sObject Quick Action Details
//! - sObject Quick Action Default Values
//!     - Get sObject Quick Action Default Values
//!     - Return Headers Using sObject Quick Action Default Values
//! - sObject Quick Action Default Values by ID
//!     - Get sObject Quick Action Default Values by ID
//!     - Return Headers Using sObject Quick Action Default Values by ID
//! - sObject Relationships
//!     - Get Records Using sObject Relationships
//!     - Update Records Using sObject Relationships
//!     - Delete Records Using sObject Relationships
//! - sObjects Suggested Articles
//! - sObjects Suggested Articles by ID
//! - sObject User Password
//!     - [Get User Password Expiration Status](crate::rest_api::RestApi::is_user_password_expired)
//!     - [Set User Password](crate::rest_api::RestApi::set_user_password)
//!     - [Reset User Password](crate::rest_api::RestApi::reset_user_password)
//!     - Return Headers Using sObject User Password
//! - sObject Self-Service User Password
//!     - Get Self-Service User Password Expiration Status
//!     - Set Self-Service User Password
//!     - Reset Self-Service User Password
//!     - Return Headers Using sObject Self-Service User Password
//! - Platform Event Schema by Event Name
//! - Platform Event Schema by Schema ID
//! - Get AppMenu Types
//! - AppMenu Items
//!     - Get AppMenu Items
//!     - Return Headers of App Menu Item Requests
//! - AppMenu Mobile Items
//!     - Get AppMenu Mobile Items
//!     - Return Headers of AppMenu Mobile Item Requests
//! - Compact Layouts
//! - ConsenT
//!     - Compile Consent Settings
//!     - Compile Multiple Types of Consent Settings
//!     - Use the Consent API with Data 360
//! - Consent Write
//! - Embedded Service Configuration Describe
//!     - Get Embedded Service Configuration
//!     - Return Headers for Embedded Service Configuration
//! - Invocable Actions
//!     - Get Invocable Actions
//!     - Return HTTP Headers for Invocable Actions
//! - Invocable Actions Custom
//!     - Deploy Data Kit Components by Using Deploy Data Kit Components Flow
//!     - Get Custom Invocable Actions
//!     - Return HTTP Headers for Custom Invocable Actions
//! - Invocable Actions Standard
//!     - Get Standard Invocable Actions
//!     - Return HTTP Headers for Standard Invocable Actions
//! - List View Basic Information
//! - List View Describe
//! - List View Results
//! - List Views for an Object
//! - Named Query APIs for SOQL Queries
//!     - Create Named Query APIs in Setup
//!     - Create Named Query APIs in VS Code
//!     - Named Query API
//! - Support Knowledge with REST API
//!     - Data Category Groups
//!     - Data Category Detail
//!     - Articles List
//!     - Articles Details
//! - Parameterized Search
//!     - Search with Parameters in the URI
//!     - Search with Parameters in the Request Body
//! - Portability
//!     - Compile Data for a Portability Request
//!     - Get the Status of Your Portability Request
//! - Process Approvals
//!     - Get Process Approvals
//!     - Submit, Approve, or Reject Process Approvals
//!     - Return HTTP Headers for Process Approvals
//! - Process Rules
//!     - Get Process Rules
//!     - Trigger Process Rules
//!     - Return HTTP Headers for Process Rules
//! - Process Rule for an sObject
//!     - Get a Process Rule for an sObject
//!     - Trigger a Process Rule for an sObject
//!     - Return HTTP Headers for a Process Rule of an sObject
//! - Process Rule List for an sObject
//!     - Get Process Rules for an sObject
//!     - Return HTTP Headers for Process Rules of an sObject
//! - Product Schedules
//!     - Get Product Schedules
//!     - Create Product Schedules
//!     - Delete Product Schedules
//! - [Query](crate::rest_api::builder::RestApiBuilder::query)
//!     - Data Cloud Query Profile Parameters
//! - [Query More Results](crate::rest_api::builder::query::query_response::QueryResponse::send_query_more)
//! - [QueryAll](crate::rest_api::builder::query::QueryBuilder::all)
//! - [QueryAll](crate::rest_api::builder::query::QueryBuilder::all) [More Results](crate::rest_api::builder::query::query_response::QueryResponse::send_query_more)
//! - Query Performance Feedback (Beta)
//! - Quick Actions
//!     - Get Quick Actions
//!     - Create Records Using Quick Actions
//!     - Return Headers of Quick Actions
//! - Recent List Views
//! - Recently Viewed Items
//! - Record Count
//!     - Record Count Response Body
//! - sObject Relevant Items
//! - Get Knowledge Language Settings
//! - [Search](crate::rest_api::builder::RestApiBuilder::sosl)
//! - Search Scope and Order
//! - Search Result Layouts
//! - Lightning Toggle Metrics
//! - Lightning Usage by App Type
//! - Lightning Usage by Browser
//! - Lightning Usage by Page
//! - Lightning Usage by FlexiPage
//! - Lightning Exit by Page Metrics
//! - Salesforce Scheduler Resources
//!     - Scheduling
//!     - Get Appointment Slots
//!     - Get Appointment Candidates
//!     - Request Bodies
//!         - Work Type
//!         - Skill Requirement
//!     - Response Bodies
//!         - Time Slot
//!         - Candidates
//! - Search for Records Suggested by Autocomplete and Instant Results
//! - Search Suggested Article Title Matches
//! - Search Suggested Queries
//! - Salesforce Surveys Translation Resources
//!     - Add or Change the Translation of a Survey Field
//!     - Add or Update the Translated Value of Multiple Survey Fields in One or More Languages
//!     - Delete the Translated Value of a Survey Field
//!     - Delete the Translated Value of Multiple Survey Fields in One or More Languages
//!     - Get Translated Value of a Survey Field
//!     - Get the Translated Values of Multiple Survey Fields in One or More Languages
//! - Tabs
//!     - Get Tabs
//!     - Return Headers Using Tabs
//! - Themes
//! - Composite
//!     - Send Multiple Requests Using Composite
//!         - Composite Request Body
//!         - Composite Response Body
//!         - Composite Subrequest Result
//!     - Get a List of Composite Resources
//! - Composite Graph
//!     - Composite Subrequest
//!     - [Composite Graph Limits](https://developer.salesforce.com/docs/platform/api-rest/guide/resources-composite-graph-limits.html)
//! - Composite Batch
//!     - Batch Request Body
//!     - Batch Response Body
//! - SObject Tree
//!     - sObject Tree Request Body
//!     - sObject Tree Response Body
//! - sObject Collections
//!     - [Create Records Using sObject Collections](crate::rest_api::builder::composite::sobject_collections::SObjectCollectionsBuilder::create)
//!     - [Get Records Using sObject Collections](crate::rest_api::builder::composite::sobject_collections::SObjectCollectionsBuilder::get_by_ids)
//!     - Get Records With a Request Body Using sObject Collections
//!     - Update Records Using sObject Collections
//!     - Upsert Records Using sObject Collections
//!     - Delete Records Using sObject Collections

use std::collections::HashMap;
use std::fmt::Debug;
use log::trace;
use reqwest::Response;
use rustsf_auth::SalesforceCredentials;
use serde::de::DeserializeOwned;
use serde::Serialize;
use client::Client;
use crate::ErrorResponse;
use crate::rest_api::error::RestApiError;

pub mod builder;
pub mod client;
pub mod error;
pub mod responses;

pub const DEFAULT_API_VERSION: &str = "68.0";

pub enum HTTPMethod {
	GET,
	POST,
	// PUT,
	// DELETE,
	PATCH,
	// OPTIONS,
	// HEAD,
	// CONNECT,
}

/// A `RestApi` struct that represents the core component for interacting with a RESTful API.
///
/// This struct contains a `client` field, which is an instance of the `Client` type
/// used to make HTTP requests to the API. The `RestApi` struct is derived with the
/// `Default` trait, allowing it to be instantiated with default values.
///
/// # Fields
/// - `client` (`Client`):
///   A structure responsible for handling HTTP interactions with the API.
///   This field has module-level visibility (`pub(crate)`), restricting its accessibility
///   to the current crate.
///
/// Note: To use the `RestApi` struct effectively, ensure all required components
/// of the `Client` type are configured appropriately.
pub struct RestApi {
	pub(crate) client: Client,
}

impl RestApi {
	/// Creates a new instance of the `RestApi` struct with the provided `Client`.
	///
	/// # Arguments
	///
	/// * `client` - An instance of the `Client` that this `RestApi` will use to perform API requests.
	///
	/// # Returns
	///
	/// A new `RestApi` instance initialized with the given `Client`.
	///
	/// # Examples
	///
	/// ```
	/// use rustsf::{AuthUrl, RestApi};
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///
	///     let mut api = RestApi::new(SalesforceCredentials::sfdx_url("force://....my.salesforce.comm")?).await?;
	///     // you call the api here..
	///
	///     Ok(())
	/// }
	/// ```
	pub async fn new(credentials: SalesforceCredentials) -> Result<Self, RestApiError> {
		Ok(Self {
			client: Client::new(credentials).await?
		})
	}

	/// List Available REST Resources
	///
	/// # Returns
	///
	/// A `Result` containing a `HashMap<String, String>` on success, where the keys and values
	/// represent the resource identifiers and their corresponding details, respectively.
	/// If the operation fails, an `Error` is returned.
	///
	/// # Errors
	///
	/// This function will return an `Error` if:
	/// - The base path or version cannot be retrieved from the client.
	/// - The HTTP GET request fails.
	/// - The response cannot be parsed as a valid JSON.
	///
	/// # Example
	///
	/// ```rust
	/// use rustsf::{RestApi, SalesforceCredentials};
	/// use std::collections::HashMap;
	/// use anyhow::Result;
	///
	/// #[tokio::main]
	/// async fn main() -> Result<()> {
	///     let credentials = SalesforceCredentials::sfdx_url("force://PlatformCLI::token@org-name.my.salesforce.com").unwrap();
	///
	///     let api = RestApi::new(credentials).await?;
	///     match api.list_resources().await {
	///         Ok(data) => println!("Resources: {:?}", data),
	///         Err(e) => eprintln!("Failed to list resources: {}", e),
	///     }
	///     Ok(())
	/// }
	/// ```
	///
	/// # See
	/// https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/dome_discoveryresource.htm
	pub async fn list_resources(&self) -> Result<HashMap<String, String>, RestApiError> {
		let url = self.client.services_data_version_path();
		let params: Vec<(String, String)> = vec![];
		let response = self.client.get(&url, &params, &HashMap::new()).await?;
		RestApi::handle_json_response(response).await
	}

	pub fn org_id(&self) -> Option<&str> {
		self.client.user_id()
	}

	pub fn user_id(&self) -> Option<&str> {
		self.client.user_id()
	}

	pub(crate) async fn send_delete(
		&self,
		url: String,
		headers: &HashMap<String, String>,
	) -> Result<reqwest::Response, RestApiError> {
		let url = format!("{}/{}", self.client.instance_url(), url);
		self.client.delete(&url, &headers).await
	}

	pub(crate) async fn send<T: Debug + Serialize>(
		&self,
		method: HTTPMethod,
		url: String,
		data: &T,
		headers: &HashMap<String, String>,
	) -> Result<reqwest::Response, RestApiError> {
		let url = format!("{}/{}", self.client.instance_url(), url);
		match method {
			HTTPMethod::GET => Ok(self.client.get(&url, data, &headers).await?),
			HTTPMethod::POST => Ok(self.client.post(&url, data, &headers).await?),
			HTTPMethod::PATCH => Ok(self.client.patch(&url, data, &headers).await?),
		}
	}

	async fn handle_json_response<T: DeserializeOwned>(response: Response) -> Result<T, RestApiError> {

		trace!("Headers: {:?}", response.headers());

		if response.status().is_success() {
			let json_str = response.text().await?;
			trace!("Raw response: {}", json_str);
			Ok(serde_json::from_str(&json_str)?)
		//	Ok(response.json().await?)
		} else {
			let errors: Vec<ErrorResponse> = response.json().await?;
			Err(RestApiError::Salesforce(errors))
		}
	}

	async fn handle_empty_response(response: Response) -> Result<(), RestApiError> {
		if response.status().is_success() {
			Ok(())
		} else {
			let errors: Vec<ErrorResponse> = response.json().await?;
			Err(RestApiError::Salesforce(errors))
		}
	}

	// todo add method for unsupported paths
}



#[cfg(test)]
mod test;

