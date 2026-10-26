//! Example on how to create a record using the Salesforce Rest API.
//!
//! This example is similar to the `create` example but uses a stronger typed method to avoid runtime errors.
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct NewAccount {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Account {
    id: String,
    name: String,
}

impl Account {
    pub fn from(id: String, new_account: NewAccount) -> Self {
        Self {
            id,
            name: new_account.name,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let auth_url = env::var("SFDC_AUTH_URL").unwrap();
    let credentials = SalesforceCredentials::sfdx_url(auth_url).unwrap();
    let api = RestApi::new(credentials).await.unwrap();

    let new_account = NewAccount {
        name: "Test Account".to_string(),
    };

    let result = api.create("Account", &new_account).await?;
    let account = Account::from(result.id, new_account);
    println!("{:?}", account);

    Ok(())
}
