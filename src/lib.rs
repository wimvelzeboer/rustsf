//! # Salesforce SDK for Rust
//!
//! A comprehensive library that provides a high-level interface to the Salesforce APIs.
//! It allows developers to interact with Salesforce data and metadata in a structured and
//! idiomatic way, making it easier to manage and manipulate data.
//!
//! ## Features
//! All features are updated to the latest version (v67.0 Summer 2026) of the Salesforce APIs.
//!
//! - [**Rest API**](crate::rest_api)
//! - [**Tooling API**](crate::tooling_api)
//! - [**Metadata API**](crate::metadata_api)
//!
mod client;

#[cfg(feature = "rest-api")]
pub mod rest_api;
/*
#[cfg(feature = "tooling-api")]
pub mod tooling_api;

#[cfg(feature = "metadata-api")]
pub mod metadata_api;

#[cfg(feature = "bulk-api")]
pub mod bulk_api;

#[cfg(feature = "bulk-api-v2")]
pub mod bulk_api_v2;
*/
pub mod primary_types;


pub use rustsf_auth::Credentials;


pub use client::Client;
pub use client::credentials::Credentials;
pub use client::credentials::auth_url::AuthUrl;
pub use client::credentials::client_credentials::ClientCredentials;
pub use client::credentials::credential_file::CredentialFile;
pub use primary_types::{SObject, SObjectOwner};
pub use rest_api::responses::error_response::ErrorResponse;

#[cfg(feature = "rest-api")]
pub use rest_api::RestApi;
/*
#[cfg(feature = "tooling-api")]
pub use tooling_api::ToolingApi;

#[cfg(feature = "metadata-api")]
pub use metadata_api::MetadataApi;

#[cfg(feature = "bulk-api")]
pub use bulk_api::BulkApi;

#[cfg(feature = "bulk-api-v2")]
pub use bulk_api_v2::BulkApiV2;
*/
pub use rustsf_marcos::def_sobject as DefSObject;
