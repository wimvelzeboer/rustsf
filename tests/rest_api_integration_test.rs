extern crate rustsf;
use anyhow::Result;
use log::LevelFilter;

mod common;

///  Check the available API versions
#[tokio::test]
#[ignore]
async fn check_versions() -> Result<()> {
	let api = common::get_rest_api_client().await?;
	let versions = api.versions().await?;

	assert_ne!(0, versions.len());
	assert_eq!(
		rustsf::rest_api::DEFAULT_API_VERSION,
		versions[versions.len() - 1].version
	); // this forces us to always use the latest version
	Ok(())
}

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

#[tokio::test]
#[ignore]
async fn test_create_sobjects_failing() -> Result<()> {
	let api = common::get_rest_api_client().await?;

	// Create multiple Account records
	let mut accounts = vec![common::Account::new().set_name("Test Account 1".to_string())];

	let results = api.create_sobjects(&mut accounts, true).await?;

	assert_eq!(1, results.len());
	for result in &results {
		assert!(
			!result.success,
			"Did not expect that the record creation was successful"
		);
		assert!(result.id.is_none());
		for error in &result.errors {
			assert_eq!(error.status_code, "INVALID_CROSS_REFERENCE_KEY", "Invalid status code");
			assert_eq!(
				error.message, "Owner ID: owner cannot be blank",
				"Invalid error message"
			);
			assert_eq!(1, error.fields.len());
			assert_eq!(error.fields[0], "OwnerId");
		}
	}

	Ok(())
}

/// Test fetching multiple SObjects using sobject_by_ids
#[tokio::test]
#[ignore]
async fn test_sobject_by_ids() -> Result<()> {
	let mut api = common::get_rest_api_client().await?;

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

	// Extract created IDs
	let ids: Vec<&str> = results
		.iter()
		.filter_map(|result| result.id.as_ref())
		.map(String::as_str)
		.collect();

	assert_eq!(2, ids.len());

	println!("Created Account IDs: {:?}", ids);

	// Fetch the created records by IDs
	let fetched_accounts = api
		.sobject_by_ids::<common::Account>("Account", &ids, &["Id", "Name", "OwnerId"])
		.await?;

	println!("Accounts {:?}", fetched_accounts);

	assert_eq!(2, fetched_accounts.len());
	for account in &fetched_accounts {
		assert!(account.id.is_some());
		assert_eq!(18, account.id.as_ref().unwrap().len()); // Verify 18 character Salesforce Id
		assert!(account.get_name().is_some());
		assert!(account.get_name().unwrap().starts_with("Test Account"));
	}

	// Clean up created records
	for id in ids {
		api.delete_sobject("Account", &id).await?;
	}

	Ok(())
}

#[tokio::test]
#[ignore]
async fn test_assignment_rule_header_create_account() -> Result<()> {
	let api = common::get_rest_api_client().await?;

	let account = common::Account::new()
		.set_name("Assignment Rule Header Test Account")
		.set_owner_id(api.user_id().unwrap());

	let result = api
		.build()
		.assignment_rule_header("false")
		.sobjects()
		.create("Account", account)
		.send()
		.await?;

	assert!(result.is_success(), "Account creation failed");

	let id = result
		.take_id()
		.expect("Expected created Account response to include an Id");

	assert_eq!(18, id.len());

	api.delete_sobject("Account", &id).await?;

	Ok(())
}

#[tokio::test]
#[ignore]
async fn test_call_options_header_create_account() -> Result<()> {
	let api = common::get_rest_api_client().await?;

	let account = common::Account::new()
		.set_name("Call Options Header Test Account")
		.set_owner_id(api.user_id().unwrap());

	let result = api
		.build()
		.call_options_header(Some("rustsf"), None)
		.sobjects()
		.create("Account", account)
		.send()
		.await?;

	assert!(result.is_success(), "Account creation failed");

	let id = result
		.take_id()
		.expect("Expected created Account response to include an Id");

	assert_eq!(18, id.len());

	api.delete_sobject("Account", &id).await?;

	Ok(())
}

#[tokio::test]
//#[ignore]
async fn test_if_match_header_update_account_precondition() -> Result<()> {
	/*	env_logger::builder()
	.filter_level(LevelFilter::Trace)
	.format_target(false)
	.init();*/

	let api = common::get_rest_api_client().await?;

	let account = common::Account::new()
		.set_name("If-Match Header Test Account")
		.set_owner_id(api.user_id().unwrap());

	let result = api.build().sobjects().create("Account", account).send().await?;

	assert!(result.is_success(), "Account creation failed");

	let id = result
		.take_id()
		.expect("Expected created Account response to include an Id");

	assert_eq!(18, id.len());

	let response = api.build().sobjects().get_by_id("Account", &id).send().await?;
	let etag = response.etag();
	assert!(etag.is_some(), "Expected etag to be returned");

	// Incorrect etag should fail
	let update_result = api
		.build()
		.if_match_header(&["definitely-not-matching-etag"])
		.sobjects()
		.update(
			"Account",
			&id,
			serde_json::json!({
				"Description": "If-Match Header Updated Account"
			}),
		)
		.send()
		.await;

	assert!(
		update_result.is_err(),
		"Expected update with a non-matching If-Match ETag to fail"
	);

	// correct etag should succeed
	let update_result = api
		.build()
		.if_match_header(&[&etag.unwrap()])
		.sobjects()
		.update(
			"Account",
			&id,
			serde_json::json!({
				"Description": "If-Match Header Updated Account"
			}),
		)
		.send()
		.await;

	assert!(
		update_result.is_ok(),
		"Expected update with a matching If-Match ETag to succeed"
	);
	api.delete_sobject("Account", &id).await?;

	Ok(())
}

#[tokio::test]
#[ignore]
async fn test_duplicate_rule_header_create_account() -> Result<()> {
	let api = common::get_rest_api_client().await?;

	let account = common::Account::new()
		.set_name("Duplicate Rule Header Test Account")
		.set_owner_id(api.user_id().unwrap());

	let result = api
		.build()
		.duplicate_rule_header(true, false, true)
		.sobjects()
		.create("Account", account)
		.send()
		.await?;

	assert!(result.is_success(), "Account creation failed");

	let id = result
		.take_id()
		.expect("Expected created Account response to include an Id");

	assert_eq!(18, id.len());

	api.delete_sobject("Account", &id).await?;

	Ok(())
}

#[tokio::test]
#[ignore]
async fn test_most_recently_used_header_create_account() -> Result<()> {
	let api = common::get_rest_api_client().await?;

	let account = common::Account::new()
		.set_name("Most Recently Used Header Test Account")
		.set_owner_id(api.user_id().unwrap());

	let result = api
		.build()
		.most_recently_used_header(true)
		.sobjects()
		.create("Account", account)
		.send()
		.await?;

	assert!(result.is_success(), "Account creation failed");

	let id = result
		.take_id()
		.expect("Expected created Account response to include an Id");

	assert_eq!(18, id.len());

	api.delete_sobject("Account", &id).await?;

	Ok(())
}

#[tokio::test]
#[ignore]
async fn test_query_options_header_query_accounts() -> Result<()> {
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

	let mut builder = api.build();
	let result = builder
		.query_options_header(1)
		.query("SELECT Id, Name, OwnerId FROM Account")
		.send()
		.await?;

	assert!(result.total_size() > 1, "Query failed");

	Ok(())
}

#[tokio::test]
#[ignore]
async fn test_list_limits() -> Result<()> {
	let mut api = common::get_rest_api_client().await?;

	let limits = api.list_limits().await?;

	println!("Limits: {:?}", limits);

	assert!(
		limits.daily_api_requests.max > 0,
		"Expected Daily API Requests max limit to be greater than zero"
	);
	assert!(
		limits.daily_api_requests.remaining <= limits.daily_api_requests.max,
		"Expected Daily API Requests remaining limit to be less than or equal to max"
	);

	Ok(())
}

#[tokio::test]
#[ignore]
async fn test_query_all_accounts() -> Result<()> {
	let mut api = common::get_rest_api_client().await?;

	let account = common::Account::new()
		.set_name("Query All Test Account")
		.set_owner_id(api.user_id().unwrap());

	let result = api.build().sobjects().create("Account", account).send().await?;

	assert!(result.is_success(), "Account creation failed");

	let id = result
		.take_id()
		.expect("Expected created Account response to include an Id");

	assert_eq!(18, id.len());

	let response = api
		.query_all::<common::Account>(&format!("SELECT Id, Name, OwnerId FROM Account WHERE Id = '{}'", id))
		.await?;

	assert_eq!(1, response.total_size, "Expected query_all to return one Account");
	assert_eq!(1, response.records.len(), "Expected one Account record");
	assert_eq!(Some(id.as_str()), response.records[0].id.as_deref());
	assert_eq!(Some("Query All Test Account"), response.records[0].get_name());

	api.delete_sobject("Account", &id).await?;

	Ok(())
}

#[tokio::test]
#[ignore]
async fn test_search_sosl_accounts() -> Result<()> {
	let mut api = common::get_rest_api_client().await?;

	let account = common::Account::new()
		.set_name("Search Sosl Test Account")
		.set_owner_id(api.user_id().unwrap());

	let result = api.build().sobjects().create("Account", account).send().await?;

	assert!(result.is_success(), "Account creation failed");

	let id = result
		.take_id()
		.expect("Expected created Account response to include an Id");

	assert_eq!(18, id.len());

	let response = api
		.search_sosl("FIND {Search Sosl Test Account} RETURNING Account(Id)")
		.await?;

	assert!(
		response.search_records.iter().any(|record| record.id == id),
		"Expected SOSL search results to include created Account"
	);

	api.delete_sobject("Account", &id).await?;

	Ok(())
}

#[tokio::test]
#[ignore]
async fn test_create_account() -> Result<()> {
	let api = common::get_rest_api_client().await?;

	let account = common::Account::new()
		.set_name("Create Method Test Account")
		.set_owner_id(api.user_id().unwrap());

	let result = api.create("Account", account).await?;

	assert!(result.success, "Account creation failed");
	assert!(result.id.is_some(), "Expected created Account response to include an Id");

	let id = result.id.expect("Expected created Account response to include an Id");

	assert_eq!(18, id.len());

	api.delete_sobject("Account", &id).await?;

	Ok(())
}

#[tokio::test]
#[ignore]
async fn test_sobject_get_deleted_account() -> Result<()> {
	env_logger::builder()
		.filter_level(LevelFilter::Trace)
		.format_target(false)
		.init();

	let api = common::get_rest_api_client().await?;

	let start_date = chrono::Utc::now() - chrono::Duration::minutes(5);

	let account = common::Account::new()
		.set_name("Get Deleted Test Account")
		.set_owner_id(api.user_id().unwrap());

	let result = api.create("Account", account).await?;

	assert!(result.success, "Account creation failed");

	let id = result
		.id
		.expect("Expected created Account response to include an Id");

	assert_eq!(18, id.len());

	api.delete_sobject("Account", &id).await?;

	let end_date = chrono::Utc::now() + chrono::Duration::minutes(5);

	let response = api
		.sobject_get_deleted("Account", &start_date, &end_date)
		.await?;

	assert!(
		response
			.deleted_records
			.iter()
			.any(|record| record.id == id),
		"Expected deleted records to include deleted Account"
	);

	assert!(
		!response.earliest_date_available.is_empty(),
		"Expected earliest_date_available to be present"
	);
	assert!(
		!response.latest_date_covered.is_empty(),
		"Expected latest_date_covered to be present"
	);

	Ok(())
}

#[tokio::test]
#[ignore]
async fn test_describe_global() -> Result<()> {
	let api = common::get_rest_api_client().await?;

	let response = api.describe_global().await?;

	assert_eq!("UTF-8", response.encoding);
	assert!(
		response.max_batch_size > 0,
		"Expected max_batch_size to be greater than zero"
	);
	assert!(
		!response.sobjects.is_empty(),
		"Expected describe_global to return at least one sObject"
	);
	assert!(
		response
			.sobjects
			.iter()
			.any(|sobject| sobject.name == "Account"),
		"Expected describe_global response to include Account"
	);

	Ok(())
}

#[tokio::test]
#[ignore]
async fn test_sobject_basic_info_account() -> Result<()> {
	let api = common::get_rest_api_client().await?;

	let response = api.sobject_basic_info("Account").await?;

	assert_eq!("Account", response.object_describe.name);
	assert!(
		response.object_describe.queryable,
		"Expected Account to be queryable"
	);
	assert!(
		response
			.object_describe
			.urls
			.contains_key("describe"),
		"Expected Account basic info to include describe URL"
	);

	Ok(())
}

#[tokio::test]
#[ignore]
async fn test_sobject_by_ext_id_account() -> Result<()> {
	let api = common::get_rest_api_client().await?;

	let account = common::Account::new()
		.set_name("External Id Lookup Test Account")
		.set_owner_id(api.user_id().unwrap());

	let result = api.create("Account", account).await?;

	assert!(result.success, "Account creation failed");
	assert!(result.id.is_some(), "Expected created Account response to include an Id");

	let id = result.id.expect("Expected created Account response to include an Id");

	assert_eq!(18, id.len());

	let fetched_account = api
		.sobject_by_ext_id::<common::Account>("Account", "Id", &id)
		.await?;

	assert_eq!(Some(id.as_str()), fetched_account.id.as_deref());
	assert_eq!(
		Some("External Id Lookup Test Account"),
		fetched_account.get_name()
	);

	api.delete_sobject("Account", &id).await?;

	Ok(())
}

#[tokio::test]
#[ignore]
async fn test_sobject_by_id_account() -> Result<()> {
	let api = common::get_rest_api_client().await?;

	let account = common::Account::new()
		.set_name("SObject By Id Test Account")
		.set_owner_id(api.user_id().unwrap());

	let result = api.create("Account", account).await?;

	assert!(result.success, "Account creation failed");
	assert!(result.id.is_some(), "Expected created Account response to include an Id");

	let id = result.id.expect("Expected created Account response to include an Id");

	assert_eq!(18, id.len());

	let fetched_account = api.sobject_by_id::<common::Account>("Account", &id).await?;

	assert_eq!(Some(id.as_str()), fetched_account.id.as_deref());
	assert_eq!(
		Some("SObject By Id Test Account"),
		fetched_account.get_name()
	);

	api.delete_sobject("Account", &id).await?;

	Ok(())
}

#[tokio::test]
#[ignore]
async fn test_sobject_describe_account() -> Result<()> {
	let mut api = common::get_rest_api_client().await?;

	let response = api.sobject_describe("Account").await?;

	assert_eq!("Account", response.name);
	assert!(response.queryable, "Expected Account to be queryable");
	assert!(
		response.urls.contains_key("rowTemplate"),
		"Expected Account describe response to include rowTemplate URL"
	);
	assert!(
		response.fields.iter().any(|field| field.name == "Name"),
		"Expected Account describe response to include Name field"
	);

	Ok(())
}

/*
/// Create an Account record and Fetch it via Id
#[tokio::test]
#[ignore]
async fn create_fetch_delete_sobject() -> Result<()> {
	let mut client = common::get_rest_api_client().await?;

	// Create an Account record
	let account = client
		.create_owned_sobject(common::Account::new().set_name("Test Account".to_string()))
		.await?;
	assert!(account.id.is_some());
	assert_eq!(18, account.id.as_ref().unwrap().len()); // We got a 18 character Salesforce Id

	// Fetch the Account record
	let account = client
		.fetch_by_id::<common::Account>("Account", account.id.as_ref().unwrap())
		.await?;
	assert_eq!("Test Account", account.get_name().unwrap());

	client.delete_sobject("Account", &account.id.clone().unwrap()).await?;

	match client
		.fetch_by_id::<common::Account>("Account", account.id.as_ref().unwrap())
		.await
	{
		Ok(_) => panic!("Account record should have been deleted"),
		Err(_) => (), // fixme assert the right error message
	};

	Ok(())
}*/
/*
/// Fetch the Account record via SOQL


/// Create an Account record and Fetch it via Id


/// FETCH all Account records via SOQL and delete them

 */
