use rustsf::{AuthUrl, Error, RestApi};
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let auth_url = env::var("SFDC_AUTH_URL").unwrap();
    let session = SalesforceCredentials::sfdx_url(auth_url).unwrap().connect().await?;
    let api = RestApi::new(session).await.unwrap();

    api.update("Account", "0012K00001drfGYQAY", json!({"Name": "hello rust updated"})).await?;
    println!("Update successful");

    Ok(())
}
