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

    let query_result = api
        .query::<Account>("SELECT Id, Name FROM Account WHERE Id = '0012K00001drfGYQAY'")
        .await?;
    println!("{:?}", query_result);

    Ok(())
}
