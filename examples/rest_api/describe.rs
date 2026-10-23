use rustsf::{RestApi, SalesforceCredentials};
use std::env;

#[tokio::main]
async fn main()  {
    let login_url = env::var("SFDC_LOGIN_URL").unwrap();
    let client_id = env::var("SFDC_CLIENT_ID").unwrap();
    let client_secret = env::var("SFDC_CLIENT_SECRET").unwrap();

    let credentials = SalesforceCredentials::client_credentials(login_url, client_id, client_secret);

    let mut api = RestApi::new(credentials).await.unwrap();

    let body = api.describe_global().await?;
    println!("{:?}", body);

    Ok(())
}
