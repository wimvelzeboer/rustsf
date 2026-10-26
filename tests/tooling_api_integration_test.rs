extern crate rustsf;
use anyhow::Result;

mod common;

/// Test creating multiple SObjects using create_sobjects
#[tokio::test]
#[ignore]
async fn test_create_sobjects() -> Result<()> {
	let api = common::get_rest_api_client().await?;

	// Create multiple Account records
	let mut accounts = vec![
		common::Account::new()
			.set_name("Test Account 1".to_string())
			.set_owner_id(api.user_id().unwrap()),
		common::Account::new()
			.set_name("Test Account 2".to_string())
			.set_owner_id(api.user_id().unwrap()),
	];

	let results = api.create_sobjects(&mut accounts, true).await?;

	assert_eq!(2, results.len());
	for result in &results {
		assert!(result.success, "Record creation failed");
		assert!(result.id.is_some());
		assert_eq!(18, result.id.as_ref().unwrap().len()); // Verify 18 character Salesforce Id
		assert!(result.success);
	}

	// Clean up created records
	for result in results {
		if let Some(id) = result.id {
			api.delete_sobject("Account", &id).await?;
		}
	}

	Ok(())
}