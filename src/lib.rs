//! # Salesforce SDK for Rust
//!
//! The Salesforce SDK for Rust is a comprehensive library that provides a high-level interface to
//! the Salesforce APIs. It allows developers to interact with Salesforce data and metadata in a
//! structured and idiomatic way, making it easier to manage and manipulate data.
//!
//! ## Features
//!
//! - [**Rest API**](crate::rest_api), version 67.0, Summer 2026
//! - [**Tooling API**](crate::tooling_api), version 67.0, Summer 2026
//!
//! ## Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rustsf = { version = "0.0.2", features = ["rest-api", "tooling-api"] }
//!```
pub mod client;
pub mod errors;

#[cfg(feature = "rest-api")]
pub mod rest_api;

#[cfg(feature = "tooling-api")]
pub mod tooling_api;

pub mod bulk_api;
pub mod bulk_api_v2;
pub mod primary_types;

pub use client::client::Client;

#[cfg(feature = "rest-api")]
pub use rest_api::RestApi;

#[cfg(feature = "tooling-api")]
pub use tooling_api::ToolingApi;

pub use bulk_api::BulkApi;
pub use bulk_api_v2::BulkApiV2;
pub use errors::Error;

pub use rustsf_marcos::def_sobject as DefSObject;
