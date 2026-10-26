use rustsf::{AuthUrl, Error, RestApi};
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let auth_url = env::var("SFDC_AUTH_URL").unwrap();
    let credentials = SalesforceCredentials::sfdx_url(auth_url).unwrap();
    let api = RestApi::new(credentials).await.unwrap();

    let res = api
        .upsert("Account", "ExKey__c", "0012K00001drfGYQAY1", json!({"Name": "hello rust"}))
        .await?;
    println!("{:?}", res);

    Ok(())
}
