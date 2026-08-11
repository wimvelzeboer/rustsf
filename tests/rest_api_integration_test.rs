extern crate rustsf;
use anyhow::Result;
use crate::rustsf::primary_types::SObject;

mod common;


///  Check the available API versions
#[tokio::test]
async fn check_versions() -> Result<()> {
    let mut client = common::get_rest_api_client().await?;
    let versions = client.api_versions().await?;

    assert_ne!(0, versions.len());
    assert_eq!("67.0", versions[versions.len()-1].version); // this forces us to always use the latest version
    Ok(())
}

/// Create an Account record and Fetch it via Id
#[tokio::test]
async fn create_fetch_delete_sobject() -> Result<()> {
    let mut client = common::get_rest_api_client().await?;

    // Create an Account record
    let account = client.create_sobject(common::Account::new()
        .set_name("Test Account".to_string())).await?;
    assert!(account.id().is_some());
    assert_eq!(18, account.id().unwrap().len()); // We got a 18 character Salesforce Id

    // Fetch the Account record
    let account = client.fetch_by_id::<common::Account>("Account", &account.id().unwrap()).await?;
    assert_eq!("Test Account", account.get_name().unwrap());

    client.delete_sobject("Account", &account.id().unwrap()).await?;

    match client.fetch_by_id::<common::Account>("Account", &account.id().unwrap()).await {
        Ok(_) => panic!("Account record should have been deleted"),
        Err(_) => (),  // fixme assert the right error message
    };

    Ok(())
}
/*
/// Fetch the Account record via SOQL


/// Create an Account record and Fetch it via Id


/// FETCH all Account records via SOQL and delete them

 */