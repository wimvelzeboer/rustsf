use super::*;
use mockito::Server;
use serde_json::json;

fn create_test_tooling_api(server_url: &str) -> ToolingApi {
	let mut client = Client::new(Credentials::new_mock()).await?;
	client.set_instance_url(server_url);
	client.set_access_token(
		"test_token".to_string(),
		"9999999999000".to_string(),
		"Bearer".to_string(),
	);
	client.set_version("v60.0");
	ToolingApi::new(client)
}

#[test]
fn test_new() {
	let client = Client::new(Credentials::new_mock()).await?;
	let api = ToolingApi::new(client);
	assert!(api.client.instance_url.is_none());
}

#[test]
fn test_base_path() {
	let mut client = Client::new(Credentials::new()).await?;
	client.set_instance_url("https://na1.salesforce.com");
	client.set_version("v60.0");
	let api = ToolingApi::new(client);
	assert_eq!(
		api.base_path().unwrap(),
		"https://na1.salesforce.com/services/data/v60.0/tooling"
	);
}

// ── Execute Anonymous ───────────────────────────────────────────

#[tokio::test]
async fn test_execute_anonymous_success() {
	let mut server = Server::new_async().await;
	let mock = server
		.mock("GET", "/services/data/v60.0/tooling/executeAnonymous/")
		.match_query(mockito::Matcher::UrlEncoded(
			"anonymousBody".into(),
			"System.debug('Hello');".into(),
		))
		.with_status(200)
		.with_header("content-type", "application/json")
		.with_body(
			json!({
				"compiled": true,
				"compileProblem": null,
				"success": true,
				"line": -1,
				"column": -1,
				"exceptionMessage": null,
				"exceptionStackTrace": null
			})
			.to_string(),
		)
		.create_async()
		.await;

	let mut api = create_test_tooling_api(&server.url());
	let result = api.execute_anonymous("System.debug('Hello');").await.unwrap();
	assert!(result.compiled);
	assert!(result.success);
	assert_eq!(result.line, -1);
	assert_eq!(result.column, -1);
	assert!(result.compile_problem.is_none());
	assert!(result.exception_message.is_none());
	mock.assert_async().await;
}

#[tokio::test]
async fn test_execute_anonymous_compile_error() {
	let mut server = Server::new_async().await;
	let mock = server
		.mock("GET", "/services/data/v60.0/tooling/executeAnonymous/")
		.match_query(mockito::Matcher::UrlEncoded(
			"anonymousBody".into(),
			"invalid apex code".into(),
		))
		.with_status(200)
		.with_header("content-type", "application/json")
		.with_body(
			json!({
				"compiled": false,
				"compileProblem": "Unexpected token 'invalid'.",
				"success": false,
				"line": 1,
				"column": 1,
				"exceptionMessage": null,
				"exceptionStackTrace": null
			})
			.to_string(),
		)
		.create_async()
		.await;

	let mut api = create_test_tooling_api(&server.url());
	let result = api.execute_anonymous("invalid apex code").await.unwrap();
	assert!(!result.compiled);
	assert!(!result.success);
	assert_eq!(result.compile_problem.as_deref(), Some("Unexpected token 'invalid'."));
	assert_eq!(result.line, 1);
	mock.assert_async().await;
}

#[tokio::test]
async fn test_execute_anonymous_runtime_error() {
	let mut server = Server::new_async().await;
	let mock = server
		.mock("GET", "/services/data/v60.0/tooling/executeAnonymous/")
		.match_query(mockito::Matcher::UrlEncoded(
			"anonymousBody".into(),
			"Integer x = 1/0;".into(),
		))
		.with_status(200)
		.with_header("content-type", "application/json")
		.with_body(
			json!({
				"compiled": true,
				"compileProblem": null,
				"success": false,
				"line": 1,
				"column": 1,
				"exceptionMessage": "System.MathException: Divide by 0",
				"exceptionStackTrace": "AnonymousBlock: line 1, column 1"
			})
			.to_string(),
		)
		.create_async()
		.await;

	let mut api = create_test_tooling_api(&server.url());
	let result = api.execute_anonymous("Integer x = 1/0;").await.unwrap();
	assert!(result.compiled);
	assert!(!result.success);
	assert_eq!(
		result.exception_message.as_deref(),
		Some("System.MathException: Divide by 0")
	);
	assert_eq!(
		result.exception_stack_trace.as_deref(),
		Some("AnonymousBlock: line 1, column 1")
	);
	mock.assert_async().await;
}

#[tokio::test]
async fn test_execute_anonymous_http_error() {
	let mut server = Server::new_async().await;
	let mock = server
		.mock("GET", "/services/data/v60.0/tooling/executeAnonymous/")
		.match_query(mockito::Matcher::UrlEncoded(
			"anonymousBody".into(),
			"System.debug('test');".into(),
		))
		.with_status(401)
		.with_header("content-type", "application/json")
		.with_body(
			json!([{
				"message": "Session expired or invalid",
				"errorCode": "INVALID_SESSION_ID"
			}])
			.to_string(),
		)
		.create_async()
		.await;

	let mut api = create_test_tooling_api(&server.url());
	let result = api.execute_anonymous("System.debug('test');").await;
	assert!(result.is_err());
	mock.assert_async().await;
}

// ── Debug Logs ──────────────────────────────────────────────────

#[tokio::test]
async fn test_get_latest_apex_logs() {
	let mut server = Server::new_async().await;
	let mock = server
		.mock("GET", "/services/data/v60.0/tooling/query/")
		.match_query(mockito::Matcher::UrlEncoded(
			"q".into(),
			"SELECT Id FROM ApexLog ORDER BY StartTime DESC LIMIT 1".into(),
		))
		.with_status(200)
		.with_header("content-type", "application/json")
		.with_body(
			json!({
				"totalSize": 1,
				"done": true,
				"records": [{"Id": "07Lxx0000001"}]
			})
			.to_string(),
		)
		.create_async()
		.await;

	let mut api = create_test_tooling_api(&server.url());
	let result = api.get_latest_apex_logs(1).await.unwrap();
	assert_eq!(result["totalSize"], 1);
	assert_eq!(result["records"][0]["Id"], "07Lxx0000001");
	mock.assert_async().await;
}

#[tokio::test]
async fn test_get_apex_log_body() {
	let mut server = Server::new_async().await;
	let log_body = "48.0 APEX_CODE,FINEST\nExecute Anonymous: System.debug('Hello');\n|DEBUG|Hello";
	let mock = server
		.mock("GET", "/services/data/v60.0/sobjects/ApexLog/07Lxx0000001/Body")
		.with_status(200)
		.with_header("content-type", "text/plain")
		.with_body(log_body)
		.create_async()
		.await;

	let mut api = create_test_tooling_api(&server.url());
	let result = api.get_apex_log_body("07Lxx0000001").await.unwrap();
	assert!(result.contains("|DEBUG|Hello"));
	mock.assert_async().await;
}

#[tokio::test]
async fn test_get_apex_log_body_not_found() {
	let mut server = Server::new_async().await;
	let mock = server
		.mock("GET", "/services/data/v60.0/sobjects/ApexLog/07Lxx_bad/Body")
		.with_status(404)
		.with_header("content-type", "application/json")
		.with_body(
			json!([{
				"message": "Provided external ID field does not exist",
				"errorCode": "NOT_FOUND"
			}])
			.to_string(),
		)
		.create_async()
		.await;

	let mut api = create_test_tooling_api(&server.url());
	let result = api.get_apex_log_body("07Lxx_bad").await;
	assert!(result.is_err());
	mock.assert_async().await;
}

// ── Trace Flag Management ───────────────────────────────────────

#[tokio::test]
async fn test_get_current_user_id() {
	let mut server = Server::new_async().await;
	let mock = server
		.mock("GET", "/services/data/v60.0/chatter/users/me")
		.with_status(200)
		.with_header("content-type", "application/json")
		.with_body(
			json!({
				"id": "005xx000001Svs8",
				"username": "user@example.com",
				"displayName": "Test User"
			})
			.to_string(),
		)
		.create_async()
		.await;

	let mut api = create_test_tooling_api(&server.url());
	let result = api.get_current_user_id().await.unwrap();
	assert_eq!(result["id"], "005xx000001Svs8");
	mock.assert_async().await;
}

#[tokio::test]
async fn test_get_trace_flags() {
	let mut server = Server::new_async().await;
	let mock = server
		.mock("GET", "/services/data/v60.0/tooling/query/")
		.match_query(mockito::Matcher::UrlEncoded(
			"q".into(),
			"SELECT Id FROM TraceFlag WHERE TracedEntityId = '005xx000001Svs8' AND LogType = 'DEVELOPER_LOG'".into(),
		))
		.with_status(200)
		.with_header("content-type", "application/json")
		.with_body(
			json!({
				"totalSize": 1,
				"done": true,
				"records": [{"Id": "7tfxx0000001"}]
			})
			.to_string(),
		)
		.create_async()
		.await;

	let mut api = create_test_tooling_api(&server.url());
	let result = api.get_trace_flags("005xx000001Svs8").await.unwrap();
	assert_eq!(result["totalSize"], 1);
	mock.assert_async().await;
}

#[tokio::test]
async fn test_create_trace_flag() {
	let mut server = Server::new_async().await;
	let mock = server
		.mock("POST", "/services/data/v60.0/tooling/sobjects/TraceFlag")
		.with_status(201)
		.with_header("content-type", "application/json")
		.with_body(json!({"id": "7tfxx0000002", "success": true}).to_string())
		.create_async()
		.await;

	let mut api = create_test_tooling_api(&server.url());
	let mut params = std::collections::HashMap::new();
	params.insert("TracedEntityId", "005xx000001Svs8");
	params.insert("DebugLevelId", "7dl000000000001");
	params.insert("LogType", "DEVELOPER_LOG");
	let result = api.create_trace_flag(params).await.unwrap();
	assert_eq!(result["id"], "7tfxx0000002");
	mock.assert_async().await;
}

#[tokio::test]
async fn test_update_trace_flag() {
	let mut server = Server::new_async().await;
	let mock = server
		.mock("PATCH", "/services/data/v60.0/tooling/sobjects/TraceFlag/7tfxx0000002")
		.with_status(204)
		.create_async()
		.await;

	let mut api = create_test_tooling_api(&server.url());
	let mut params = std::collections::HashMap::new();
	params.insert("ExpirationDate", "2026-12-31T23:59:59.000+0000");
	let result = api.update_trace_flag("7tfxx0000002", params).await;
	assert!(result.is_ok());
	mock.assert_async().await;
}

#[tokio::test]
async fn test_delete_trace_flag() {
	let mut server = Server::new_async().await;
	let mock = server
		.mock("DELETE", "/services/data/v60.0/tooling/sobjects/TraceFlag/7tfxx0000002")
		.with_status(204)
		.create_async()
		.await;

	let mut api = create_test_tooling_api(&server.url());
	let result = api.delete_trace_flag("7tfxx0000002").await;
	assert!(result.is_ok());
	mock.assert_async().await;
}

// ── Debug Level Management ──────────────────────────────────────

#[tokio::test]
async fn test_get_debug_level() {
	let mut server = Server::new_async().await;
	let mock = server
		.mock("GET", "/services/data/v60.0/tooling/query/")
		.match_query(mockito::Matcher::UrlEncoded(
			"q".into(),
			"SELECT Id FROM DebugLevel WHERE DeveloperName = 'SFDataloaderDebug'".into(),
		))
		.with_status(200)
		.with_header("content-type", "application/json")
		.with_body(
			json!({
				"totalSize": 1,
				"done": true,
				"records": [{"Id": "7dl000000000001"}]
			})
			.to_string(),
		)
		.create_async()
		.await;

	let mut api = create_test_tooling_api(&server.url());
	let result = api.get_debug_level("SFDataloaderDebug").await.unwrap();
	assert_eq!(result["totalSize"], 1);
	assert_eq!(result["records"][0]["Id"], "7dl000000000001");
	mock.assert_async().await;
}

#[tokio::test]
async fn test_create_debug_level() {
	let mut server = Server::new_async().await;
	let mock = server
		.mock("POST", "/services/data/v60.0/tooling/sobjects/DebugLevel")
		.with_status(201)
		.with_header("content-type", "application/json")
		.with_body(json!({"id": "7dl000000000002", "success": true}).to_string())
		.create_async()
		.await;

	let mut api = create_test_tooling_api(&server.url());
	let mut params = std::collections::HashMap::new();
	params.insert("DeveloperName", "SFDataloaderDebug");
	params.insert("MasterLabel", "SFDataloaderDebug");
	params.insert("ApexCode", "FINEST");
	params.insert("Visualforce", "NONE");
	let result = api.create_debug_level(params).await.unwrap();
	assert_eq!(result["id"], "7dl000000000002");
	mock.assert_async().await;
}

// ── Generic CRUD ────────────────────────────────────────────────

#[tokio::test]
async fn test_tooling_query() {
	let mut server = Server::new_async().await;
	let mock = server
		.mock("GET", "/services/data/v60.0/tooling/query/")
		.match_query(mockito::Matcher::UrlEncoded(
			"q".into(),
			"SELECT Id, Name FROM ApexClass LIMIT 5".into(),
		))
		.with_status(200)
		.with_header("content-type", "application/json")
		.with_body(
			json!({
				"totalSize": 2,
				"done": true,
				"records": [
					{"Id": "01pxx0000001", "Name": "MyClass"},
					{"Id": "01pxx0000002", "Name": "MyOtherClass"}
				]
			})
			.to_string(),
		)
		.create_async()
		.await;

	let mut api = create_test_tooling_api(&server.url());
	let result = api.query("SELECT Id, Name FROM ApexClass LIMIT 5").await.unwrap();
	assert_eq!(result["totalSize"], 2);
	assert_eq!(result["records"].as_array().unwrap().len(), 2);
	mock.assert_async().await;
}

#[tokio::test]
async fn test_find_by_id() {
	let mut server = Server::new_async().await;
	let mock = server
		.mock("GET", "/services/data/v60.0/tooling/sobjects/ApexClass/01pxx0000001")
		.with_status(200)
		.with_header("content-type", "application/json")
		.with_body(json!({"Id": "01pxx0000001", "Name": "MyClass", "Body": "public class MyClass {}"}).to_string())
		.create_async()
		.await;

	let mut api = create_test_tooling_api(&server.url());
	let result = api.find_by_id("ApexClass", "01pxx0000001").await.unwrap();
	assert_eq!(result["Name"], "MyClass");
	mock.assert_async().await;
}
