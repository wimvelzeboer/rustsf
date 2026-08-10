//! # Salesforce SDK for Rust
//!
//! The Salesforce SDK for Rust is a comprehensive library that provides a high-level interface to
//! the Salesforce APIs. It allows developers to interact with Salesforce data and metadata in a
//! structured and idiomatic way, making it easier to manage and manipulate data.
//!
//! ## Features
//!
//! - [**Rest API**](crate::rest_api), version 67.0, Summer 2026
//! - **Bulk API**: Provides access to the Salesforce Bulk API for uploading and downloading data in bulk.
//! - **Bulk API v2**: Provides access to the Salesforce Bulk API v2 for uploading and downloading data in bulk.
//!
pub mod client;
pub mod errors;
pub mod rest_api;

pub mod bulk_api;
pub mod bulk_api_v2;
pub mod primary_types;

pub use client::client::Client;
pub use rest_api::RestApi;
pub use bulk_api::BulkApi;
pub use bulk_api_v2::BulkApiV2;
pub use errors::Error;

pub use rustsf_marcos::def_sobject as DefSObject;
