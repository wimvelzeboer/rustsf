//! Example on how to query all record using the Rest API.
//! This includes soft-deleted records.
//!
//! Make sure to add the feature `rest-api` to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! rustsf = { version = "0.0.2", features = ["rest-api"] }
//! ```

use rustsf::{AuthUrl, Error, RestApi};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Account {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Name")]
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let auth_url = env::var("SFDC_AUTH_URL").unwrap();
    let credentials = SalesforceCredentials::sfdx_url(auth_url).unwrap();
    let api = RestApi::new(credentials).await.unwrap();
    // query_all will return also soft-deleted records in a batch of 2000.
    let query_result = api
        .query_all::<Account>("SELECT Id, Name FROM Account")
        .await?;
    println!("{:?}", query_result);

    Ok(())
}
