//! # Module containing the `DeployResult` struct
//!
//! The `DeployResult` struct represents the result of a deployment request.
//! It contains information about the deployment process, such as its status,
//! success or failure, and any errors that occurred during the deployment.
//!

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "deployOptions")]
pub struct DeployResult {
    /// The ID of the user who canceled the deployment.
    #[serde(rename = "canceledBy")]
    canceled_by: Option<String>,

    /// The full name of the user who canceled the deployment.
    #[serde(rename = "canceledByName")]
    canceled_by_name: Option<String>,

    /// Indicates whether this deployment is used to check the validity of the deployed files
    /// without changing the org (true) or not (false). A check-only deployment doesn’t deploy
    /// any components or change the organization in any way.
    #[serde(rename = "checkOnly")]
    check_only: bool,

    /// completedDate
    #[serde(rename = "completedDate")]
    completed_date: Option<DateTime<Utc>>,

    /// The ID of the user who created the deployment.
    #[serde(rename = "createdBy")]
    created_by: Option<String>,

    /// The full name of the user who created the deployment.
    #[serde(rename = "createdByName")]
    created_by_name: Option<String>,

    /// Timestamp for when the deploy request was received.
    #[serde(rename = "createdDate")]
    created_date: Option<DateTime<Utc>>,

    #[serde(rename = "deployExtensionResults")]
    deploy_extension_results: Option<Vec<String>>,

    /// Provides the details of a deployment that is in-progress or ended if ?includeDetails=true
    /// is added as a query to the GET request.
    details: Option<DeployDetails>, // todo

    /// Indicates whether the server finished processing the deploy request for the specified id.
    done: bool,

    /// Message corresponding to the values in the errorStatusCode field, if any.
    #[serde(rename = "errorMessage")]
    error_message: Option<String>,

    /// If an error occurred during the deploy request, a status code is returned, and the message
    /// corresponding to the status code is returned in errorMessagefield.
    #[serde(rename = "errorStatusCode")]
    error_status_code: Option<String>,

    /// ID of the component being deployed.
    id: String,

    /// Defaults to false. Specifies whether a deployment continues even if the deployment generates
    /// warnings. Don’t set this argument to true for deployments to production organizations.
    #[serde(rename = "ignoreWarnings")]
    ignore_warnings: bool,

    /// Timestamp of the last update for the deployment process.
    #[serde(rename = "lastModifiedDate")]
    last_modified_date: Option<DateTime<Utc>>,

    /// The number of components deployed in the deployment process. Use this value with the
    /// numberComponentsTotal value to get an estimate of the deployment’s progress.
    #[serde(rename = "numberComponentErrors")]
    number_component_errors: Option<u32>,

    #[serde(rename = "numberComponentsDeployed")]
    number_components_deployed: Option<u32>,

    /// The total number of components in the deployment. Use this value with the
    /// numberComponentsDeployed value to get an estimate of the deployment’s progress
    #[serde(rename = "numberComponentsTotal")]
    number_components_total: Option<u32>,

    #[serde(rename = "numberFiles")]
    number_files: Option<u32>,

    /// The number of Apex tests that have generated errors during this deployment.
    #[serde(rename = "numberTestErrors")]
    number_test_errors: Option<u32>,

    /// The number of completed Apex tests for this deployment. Use this value with the
    /// numberTestsTotal value to get an estimate of the deployment’s test progress.
    #[serde(rename = "numberTestsCompleted")]
    number_tests_completed: Option<u32>,

    /// The total number of Apex tests for this deployment. Use this value with the
    /// numberTestsCompleted value to get an estimate of the deployment’s test progress.
    /// The value in this field isn’t accurate until the deployment has started running
    /// tests for the components being deployed.
    #[serde(rename = "numberTestsTotal")]
    number_tests_total: Option<u32>,

    /// Defaults to true. Indicates whether any failure causes a complete rollback (true) or not
    /// (false). If false, whatever set of actions can be performed without errors are performed,
    /// and errors are returned for the remaining actions. This parameter must be set to true
    /// if you’re deploying to a production org.
    #[serde(rename = "rollbackOnError")]
    rollback_on_error: bool,

    /// Indicates whether Apex tests were run as part of this deployment (true) or not (false).
    /// Tests are either automatically run as part of a deployment or can be set to run in
    /// the deployOptions child object.
    #[serde(rename = "runTestsEnabled")]
    run_tests_enabled: bool,

    /// Timestamp for when the deployment process began.
    #[serde(rename = "startDate")]
    start_date: Option<DateTime<Utc>>,

    /// Indicates which component is being deployed or which Apex test class is running.
    #[serde(rename = "stateDetail")]
    state_detail: Option<String>,

    /// Indicates the current state of the deployment.
    ///
    /// # See
    /// [DeploymentStatus]
    status: DeploymentStatus,

    /// Indicates whether the deployment was successful (true) or not (false).
    success: bool,

    #[serde(rename = "zipSize")]
    zip_size: Option<u32>,
}

impl DeployResult {
    pub fn new() -> Self {
        Self {
            canceled_by: None,
            canceled_by_name: None,
            check_only: false,
            completed_date: None,
            created_by: None,
            created_by_name: None,
            created_date: None,
            deploy_extension_results: None,
            details: None,
            done: false,
            error_message: None,
            error_status_code: None,
            id: "".to_string(),
            ignore_warnings: false,
            last_modified_date: None,
            number_component_errors: None,
            number_components_deployed: None,
            number_components_total: None,
            number_files: None,
            number_test_errors: None,
            number_tests_completed: None,
            number_tests_total: None,
            rollback_on_error: false,
            run_tests_enabled: false,
            start_date: None,
            state_detail: None,
            status: DeploymentStatus::Pending,
            success: false,
            zip_size: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeployDetails {

    #[serde(rename = "componentFailures")]
    component_failures: Vec<ComponentDetails>,

    #[serde(rename = "componentSuccesses")]
    component_successes: Vec<ComponentDetails>,

    #[serde(rename = "retrieveResult")]
    retrieve_result: Option<String>,            // Fixme Should be RetrieveResult

    #[serde(rename = "runTestResult")]
    run_test_result: TestResults,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDetails {
    changed: bool,

    #[serde(rename = "columnNumber")]
    column_number: Option<u32>,

    #[serde(rename = "componentType")]
    component_type: String,

    created: bool,

    #[serde(rename = "createdDate")]
    created_date:  DateTime<Utc>,

    deleted: bool,

    #[serde(rename = "fileName")]
    file_name: String,

    #[serde(rename = "fullName")]
    full_name: String,

    id: Option<String>,

    #[serde(rename = "lineNumber")]
    line_number: Option<u32>,

    problem: Option<String>,

    #[serde(rename = "problemType")]
    problem_type: Option<String>,   // e.g. "Error",

    success:  bool,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct TestResults {

    #[serde(rename = "apexLogId")]
    apex_log_id: Option<String>,

    #[serde(rename = "codeCoverage")]
    code_coverage: Vec<CodeCoverage>,         // fixme

    #[serde(rename = "codeCoverageWarnings")]
    code_coverage_warnings: Vec<String>,    // fixme

    failures: Vec<String>,      // fixme

    #[serde(rename = "flowCoverage")]
    flow_coverage: Vec<String>,

    #[serde(rename = "flowCoverageWarnings")]
    flow_coverage_warnings: Vec<String>,

    #[serde(rename = "numFailures")]
    num_failures: u32,

    #[serde(rename = "num_run")]
    num_run: Option<u32>,

    #[serde(rename = "numTestsRun")]
    num_tests_run: u32,

    successes: Vec<String>, // fixme ?

    #[serde(rename = "totalTime")]
    total_time: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CodeCoverage {

    id: String,

    #[serde(rename = "locationsNotCovered")]
    locations_not_covered: Vec<CodeLocation>,

    name: String,

    namespace: Option<String>,

    #[serde(rename = "numLocations")]
    num_locations: u32,

    #[serde(rename = "numLocationsNotCovered")]
    num_locations_not_covered: u32,

    #[serde(rename = "type")]
    object_type: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct CodeLocation {

    column: Option<u32>,
    line: Option<u32>,

    #[serde(rename = "numExecutions")]
    num_executions: Option<u32>,

    time: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum DeploymentStatus {
    Pending,
    InProgress,
    FinalizingDeploy,
    FinalizingDeployFailed,
    Succeeded,
    SucceededPartial,
    Failed,
    Canceling,
    Canceled,
}
