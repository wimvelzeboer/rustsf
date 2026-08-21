//! # Salesforce SDK for Rust
//!
//! The Salesforce SDK for Rust is a comprehensive library that provides a high-level interface to
//! the Salesforce APIs. It allows developers to interact with Salesforce data and metadata in a
//! structured and idiomatic way, making it easier to manage and manipulate data.
//!
//! ## Features
//! All features are updated to the latest version (v67.0 Summer 2026) of the Salesforce APIs.
//!
//! - [**Rest API**](crate::rest_api)
//! - [**Tooling API**](crate::tooling_api)
//! - [**Metadata API**](crate::metadata_api)
//!
//! ## Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rustsf = { version = "0.0.2", features = ["rest-api", "tooling-api", "metadata_api""] }
//!```
pub mod client;
pub mod errors;

#[cfg(feature = "rest-api")]
pub mod rest_api;

#[cfg(feature = "tooling-api")]
pub mod tooling_api;

#[cfg(feature = "metadata-api")]
pub mod metadata_api;

pub mod bulk_api;
pub mod bulk_api_v2;
pub mod primary_types;

pub use client::client::Client;

#[cfg(feature = "rest-api")]
pub use rest_api::RestApi;

#[cfg(feature = "tooling-api")]
pub use tooling_api::ToolingApi;

#[cfg(feature = "metadata-api")]
pub use metadata_api::MetadataApi;

pub use bulk_api::BulkApi;
pub use bulk_api_v2::BulkApiV2;
pub use errors::Error;

pub use rustsf_marcos::def_sobject as DefSObject;
