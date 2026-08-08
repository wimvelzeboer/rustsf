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
use rustsf::{Client, Error, RestApi};
use std::collections::HashMap;
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
    let client_id = env::var("SFDC_CLIENT_ID").unwrap();
    let client_secret = env::var("SFDC_CLIENT_SECRET").unwrap();
    let username = env::var("SFDC_USERNAME").unwrap();
    let password = env::var("SFDC_PASSWORD").unwrap();

    let mut client = Client::new();
    client.set_client_id(&client_id);
    client.set_client_secret(&client_secret);
    client.login_with_credential(&username, &password).await?;

    let mut api = RestApi::new(client);

    let account = NewAccount {
        name: "Test Account".to_string(),
    };

    let account = Account::from(
        api.create("Account", &account).await?.id,
        account);
    println!("{:?}", account);

    Ok(())
}
