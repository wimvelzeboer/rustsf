use rustsf::{AuthUrl, Error, RestApi};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let auth_url = env::var("SFDC_AUTH_URL").unwrap();
    let credentials = SalesforceCredentials::sfdx_url(auth_url).unwrap();
    let api = RestApi::new(credentials).await.unwrap();

    api.delete_sobject("Account", "0011t00001FfDoaAAF").await?;
    println!("Delete successful");

    Ok(())
}
