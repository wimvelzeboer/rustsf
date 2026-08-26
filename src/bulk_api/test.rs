
use super::*;
use mockito::Server;
use serde_json::json;

fn create_test_bulk_api(server_url: &str) -> BulkApi {
    let mut client = Client::new();
    client.set_instance_url(server_url);
    client.set_access_token(
        "test_token".to_string(),
        "9999999999000".to_string(),
        "Bearer".to_string(),
    );
    client.set_version("v60.0");
    BulkApi::new(client)
}

#[test]
fn test_new() {
    let client = Client::new();
    let api = BulkApi::new(client);
    assert!(api.client.instance_url.is_none());
}

#[test]
fn test_base_path() {
    let mut client = Client::new();
    client.set_instance_url("https://na1.salesforce.com");
    client.set_version("v60.0");
    let api = BulkApi::new(client);
    // v60.0 -> strips the 'v' to get 60.0
    assert_eq!(
        api.base_path().unwrap(),
        "https://na1.salesforce.com/services/async/60.0"
    );
}

#[test]
fn test_get_auth_headers() {
    let mut client = Client::new();
    client.set_access_token(
        "my_session_token".to_string(),
        "".to_string(),
        "Bearer".to_string(),
    );
    let api = BulkApi::new(client);
    let headers = api.get_auth_headers().unwrap();
    assert_eq!(headers.len(), 1);
    assert_eq!(headers[0].0, "X-SFDC-Session");
    assert_eq!(headers[0].1, "my_session_token");
}

#[tokio::test]
async fn test_create_job() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/services/async/60.0/job")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "750xx000000001"}).to_string())
        .create_async()
        .await;

    let mut api = create_test_bulk_api(&server.url());
    let mut params = HashMap::new();
    params.insert("operation", "insert");
    params.insert("object", "Account");
    let res = api.create_job(params).await;
    assert!(res.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_add_batch_job() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/services/async/60.0/job/750xx/batch")
        .with_status(201)
        .create_async()
        .await;

    let mut api = create_test_bulk_api(&server.url());
    let csv = b"Name\nTest Account".to_vec();
    let res = api.add_batch_job("750xx", csv).await;
    assert!(res.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_batch() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/services/async/60.0/job/750xx/batch/751xx/")
        .with_status(200)
        .with_body("batch info")
        .create_async()
        .await;

    let mut api = create_test_bulk_api(&server.url());
    let res = api.get_batch("750xx", "751xx").await;
    assert!(res.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_close_job() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/services/async/60.0/job/750xx")
        .with_status(200)
        .with_body(json!({"state": "Closed"}).to_string())
        .create_async()
        .await;

    let mut api = create_test_bulk_api(&server.url());
    let res = api.close_job("750xx").await;
    assert!(res.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_job_details() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/services/async/60.0/job/750xx")
        .with_status(200)
        .with_body("job details")
        .create_async()
        .await;

    let mut api = create_test_bulk_api(&server.url());
    let res = api.get_job_details("750xx").await;
    assert!(res.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_batches() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/services/async/60.0/job/750xx/batch")
        .with_status(200)
        .with_body("batches")
        .create_async()
        .await;

    let mut api = create_test_bulk_api(&server.url());
    let res = api.get_batches("750xx", "application/json").await;
    assert!(res.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_result_list() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/services/async/60.0/job/750xx/batch/751xx/result")
        .with_status(200)
        .with_body("results")
        .create_async()
        .await;

    let mut api = create_test_bulk_api(&server.url());
    let res = api
        .get_result_list("750xx", "751xx", "application/json")
        .await;
    assert!(res.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_result() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock(
            "GET",
            "/services/async/60.0/job/750xx/batch/751xx/result/752xx",
        )
        .with_status(200)
        .with_body("result data")
        .create_async()
        .await;

    let mut api = create_test_bulk_api(&server.url());
    let res = api.get_result("750xx", "751xx", "752xx").await;
    assert!(res.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_abort_job() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/services/async/60.0/job/750xx")
        .with_status(200)
        .with_body(json!({"state": "Aborted"}).to_string())
        .create_async()
        .await;

    let mut api = create_test_bulk_api(&server.url());
    let res = api.abort_job("750xx").await;
    assert!(res.is_ok());
    mock.assert_async().await;
}