//! Example on how to query more record using the Rest API.
//! Each query call only returns 2000 records, 
//! the query_more method will return the next 2000 records.
//!
//! Make sure to add the feature `rest-api` to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! rustsf = { version = "0.0.2", features = ["rest-api"] }
//! ```

use rustsf::{Client, Error, RestApi};
use std::env;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Account {
    id: String,
    name: String,
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

    // Queries the first 2000
    let query_result = api
        .query::<Account>("SELECT Id, Name FROM Account")
        .await?;
    println!("First batch{:?}", query_result);

    // Queries the next 2000, if there is a next_records_url
    if let Some(next_records_url) = query_result.next_records_url {
        let query_result = api.query_more::<Account>(&next_records_url).await?;
        println!("Next batch{:?}", query_result);
    }

    Ok(())
}
