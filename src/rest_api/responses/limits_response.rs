//! Limits Response
//!
//! # See
//! <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_limits.htm>

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LimitsResponse {
    /// Maximum number of scratch orgs that can exist at any given time.
    pub active_scratch_orgs: Option<Limits>,

    /// Maximum amount of external data in bytes that can be uploaded daily via REST API.
    pub analytics_external_data_size: Option<Limits>,

    /// Concurrent REST API requests for asynchronous report run results.
    pub concurrent_async_get_report_instances: Limits,

    /// Concurrent Einstein Discovery data insights story creation via REST API.
    pub concurrent_einstein_data_insights_story_creation: Limits,

    /// Concurrent Einstein Discovery story creation via REST API.
    pub concurrent_einstein_discovery_story_creation: Limits,

    /// Concurrent synchronous report runs via REST API.
    pub concurrent_sync_report_runs: Limits,

    /// Daily analytics dataflow job executions via REST API.
    pub daily_analytics_dataflow_job_executions: Limits,

    /// Daily cumulative size of analytics files uploaded, in megabytes.
    pub daily_analytics_uploaded_files_size_mb: Option<Limits>,

    /// Daily API requests.
    pub daily_api_requests: Limits,

    /// Daily async Apex executions.
    pub daily_async_apex_executions: Limits,

    /// Daily async Apex tests.
    pub daily_async_apex_tests: Limits,

    /// Daily Bulk API and Bulk API 2.0 batches
    pub daily_bulk_api_batches: Limits,

    /// Daily Bulk API 2.0 query file storage, in megabytes.
    #[serde(rename = "DailyBulkV2QueryFileStorageMb")]
    pub daily_bulk_v2_query_file_storage_mb: Option<Limits>,

    /// Daily Bulk API 2.0 query jobs.
    pub daily_bulk_v2_query_jobs: Limits,

    /// Daily delivered platform events.
    pub daily_delivered_platform_events: Limits,

    /// Daily durable generic streaming API events.
    pub daily_durable_generic_streaming_api_events: Limits,

    /// Daily durable streaming API events.
    pub daily_durable_streaming_api_events: Limits,

    /// Daily Einstein Discovery data insights story creation via REST API.
    pub daily_einstein_data_insights_story_creation: Limits,

    /// Daily Einstein Discovery predict API calls.
    pub daily_einstein_discovery_predict_api_calls: Option<Limits>,

    /// Daily Einstein Discovery predictions by CDC.
    pub daily_einstein_discovery_predictions_by_cdc: Option<Limits>,

    /// Daily Einstein Discovery story creation.
    pub daily_einstein_discovery_story_creation: Limits,

    /// Daily API calls in an org with Functions.
    pub daily_functions_api_call_limit: Limits,

    /// Daily generic streaming API events.
    pub daily_generic_streaming_api_events: Limits,

    /// Daily scratch org creations you can initiate in a 24-hour window.
    pub daily_scratch_orgs: Option<Limits>,

    /// Daily standard volume platform events.
    pub daily_standard_volume_platform_events: Limits,

    /// Daily push topic event notifications delivered in the past 24 hours.
    pub daily_streaming_api_events: Limits,

    /// Daily workflow emails.
    pub daily_workflow_emails: Limits,

    /// Amount of data storage available, in megabytes.
    pub data_storage_mb: Option<Limits>,

    /// Concurrent CometD clients for durable streaming.
    pub durable_streaming_api_concurrent_clients: Limits,

    /// Amount of file storage available, in megabytes.
    pub file_storage_mb: Option<Limits>,

    /// Hourly asynchronous report runs via REST API.
    pub hourly_async_report_runs: Limits,

    /// Hourly dashboard refreshes via REST API.
    pub hourly_dashboard_refreshes: Limits,

    /// Hourly REST API requests for dashboard results.
    pub hourly_dashboard_results: Limits,

    /// Hourly dashboard status requests via REST API.
    pub hourly_dashboard_statuses: Limits,

    /// Hourly new long-term external record ID mappings.
    pub hourly_long_term_id_mapping: Limits,

    /// Hourly managed content public requests.
    pub hourly_managed_content_public_requests: Limits,

    /// Hourly OData callouts.
    pub hourly_odata_callout: Option<Limits>,

    /// Hourly published platform events.
    pub hourly_published_platform_events: Limits,

    /// Hourly published standard volume platform events.
    pub hourly_published_standard_volume_platform_events: Limits,

    /// Hourly new short-term external record ID mappings.
    pub hourly_short_term_id_mapping: Limits,

    /// Hourly synchronous report runs via REST API.
    pub hourly_sync_report_runs: Limits,

    /// Hourly workflow time triggers.
    pub hourly_time_based_workflow: Limits,

    /// Mass email.
    pub mass_email: Limits,

    /// Monthly Einstein Discovery story creation.
    pub monthly_einstein_discovery_story_creation: Limits,

    /// Org With Add-On License: Monthly Usage-Based Entitlement
    pub monthly_platform_events_usage_entitlement: Option<Limits>,

    /// Package version creates.
    pub package2_version_creates: Limits,

    /// Package version creates without validation.
    pub package2_version_creates_without_validation: Limits,

    /// Permission set limits, including custom permission sets.
    pub permission_sets: PermissionSetsLimit,

    /// Platform event triggers with parallel processing.
    pub platform_event_triggers_with_parallel_processing: Limits,

    /// Private Connect outbound callout hourly limit, in megabytes.
    pub private_connect_outbound_callout_hourly_limit_mb: Option<Limits>,

    /// Single email.
    pub single_email: Limits,

    /// Concurrent CometD clients for the streaming API.
    pub streaming_api_concurrent_clients: Limits,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Limits {
    pub max: u32,
    pub remaining: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PermissionSetsLimit {
    pub max: u32,
    pub remaining: u32,
    pub create_custom: Limits,
}
