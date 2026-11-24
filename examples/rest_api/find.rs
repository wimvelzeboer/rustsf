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
    let session = SalesforceCredentials::sfdx_url(auth_url).unwrap().connect().await?;
    let api = RestApi::new(session).await.unwrap();

    let account = api.sobject_get_by_id::<Account>("Account", "0011t00001FfE7iAAF").await?;
    println!("{:?}", account);

    Ok(())
}
