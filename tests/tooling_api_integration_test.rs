extern crate rustsf;

use anyhow::Result;
use chrono::{Duration, Utc};
use rustsf::tooling_api::schema::trace_flag::TraceFlag;
use std::collections::HashMap;

mod common;

#[tokio::test]
#[ignore]
async fn test_apex_logs() -> Result<()> {
	let api = common::get_tooling_api_client().await?;

	// Find or create debug level
	let debug_level_id = match api.get_debug_level("RustSFDebug").await?.first() {
		Some(level) => level.id().unwrap().to_string(),
		None => {
			let mut params = HashMap::new();
			params.insert("DeveloperName", "RustSFDebug");
			params.insert("MasterLabel", "RustSFDebug");
			params.insert("ApexCode", "FINEST");
			params.insert("Visualforce", "NONE");
			api.create_debug_level(params).await?
		}
	};

	// Find or create TraceFlag
	let flags = api.get_trace_flags(api.user_id().unwrap()).await?;
	let mut trace_flag: TraceFlag = match flags.into_iter().next() {
		Some(flag) => flag,
		None => {
			let mut trace_flag = TraceFlag {
				traced_entity_id: Some(api.user_id().unwrap().into()),
				debug_level_id: Some(debug_level_id.to_string()),
				log_type: Some("DEVELOPER_LOG".to_string()),
				start_date: Some(Utc::now().format("%Y-%m-%dT%H:%M:%S.000%z").to_string()),
				expiration_date: Some(Utc::now().format("%Y-%m-%dT%H:%M:%S.000%z").to_string()),
				..Default::default()
			};
			let id = api.create_trace_flag(&trace_flag).await?;
			trace_flag.set_id(Some(id));
			trace_flag
		}
	};

	// Set a new expiration date
	let exp = Utc::now() + Duration::seconds(5);
	let exp = exp.format("%Y-%m-%dT%H:%M:%S.000%z").to_string();
	trace_flag.debug_level_id = Some(debug_level_id);
	trace_flag.start_date = Some(Utc::now().format("%Y-%m-%dT%H:%M:%S.000%z").to_string());
	trace_flag.expiration_date = Some(exp);

	let id = trace_flag.id.take().unwrap();

	// Save the trace flag
	api.update_trace_flag(&id, &trace_flag).await?;

	// Execute the anonymous Apex code
	let result = api
		.execute_anonymous("System.debug('Hello from RustSF integration test');")
		.await?;

	assert!(result.compiled, "Expected anonymous Apex to compile");
	assert!(
		result.compile_problem.is_none(),
		"Did not expect an Apex compile problem"
	);
	assert!(result.success, "Expected anonymous Apex to execute successfully");
	assert_eq!(-1, result.line);
	assert_eq!(-1, result.column);
	assert!(
		result.exception_message.is_none(),
		"Did not expect an Apex runtime exception"
	);
	assert!(
		result.exception_stack_trace.is_none(),
		"Did not expect an Apex exception stack trace"
	);

	// Get the latest Apex log
	let logs = api.get_latest_apex_logs(1).await?;

	assert_eq!(logs.len(), 1, "Expected get_latest_apex_logs(1) to return one log");

	let log = logs.into_iter().next().unwrap();

	assert!(log.id().is_some(), "Expected latest ApexLog record to include an Id");
	assert!(
		log.id().unwrap().starts_with("07L"),
		"Expected ApexLog Id to start with 07L"
	);

	// get the log body
	let body = api.get_apex_log_body(log.id().unwrap()).await?;

	assert!(!body.is_empty(), "Expected ApexLog body to be non-empty");
	assert!(
		body.contains("EXECUTION_STARTED") || body.contains("Execute Anonymous"),
		"Expected ApexLog body to contain execution log content"
	);
	Ok(())
}
