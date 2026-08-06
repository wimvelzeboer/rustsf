//! Salesforce Client for authentication and bare API requests.
//!
//! This crate provides a client with several authentication types for the Salesforce APIs.
//!
#[allow(clippy::module_inception)]
pub mod client;
pub mod responses;
pub(crate) mod xml;
