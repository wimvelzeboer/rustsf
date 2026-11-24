use rustsf::{AuthUrl, Error, RestApi};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let auth_url = env::var("SFDC_AUTH_URL").unwrap();
    let session = SalesforceCredentials::sfdx_url(auth_url).unwrap().connect().await?;
    let api = RestApi::new(session).await.unwrap();

    let body = api.describe_global().await?;
    println!("{:?}", body);

    Ok(())
}
