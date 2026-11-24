use rustsf::{AuthUrl, Error, RestApi};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let auth_url = env::var("SFDC_AUTH_URL").unwrap();
    let session = SalesforceCredentials::sfdx_url(auth_url).unwrap().connect().await?;
    let api = RestApi::new(session).await.unwrap();

    api.delete_sobject("Account", "0011t00001FfDoaAAF").await?;
    println!("Delete successful");

    Ok(())
}
