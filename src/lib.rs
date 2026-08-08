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
