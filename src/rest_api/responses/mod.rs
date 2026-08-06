//! Response types for Salesforce REST API operations.
//!
//! This module contains all response structures returned by various Salesforce REST API endpoints.
//! Each submodule represents a specific API response type that can be deserialized from JSON
//! responses received from Salesforce.
//!

pub mod create_response;
pub mod describe_global_response;
pub mod describe_response;
pub mod query_response;
pub mod search_response;
pub mod version_response;
pub mod search_record;
pub mod describe_global_sobject_response;
pub mod child_relationship;
pub mod field;
pub mod urls;
pub mod sobject_attribute;
pub mod error_response;
