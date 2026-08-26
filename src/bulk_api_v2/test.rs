use super::*;
use mockito::Server;
use serde_json::json;
use crate::client::responses::access_token::AccessToken;
use crate::credentials::Credentials;

async fn create_test_bulk_api_v2(server_url: &str) -> Result<BulkApiV2> {
    let mut credentials = Credentials::new();
    credentials.set_instance_url(server_url);
    credentials.set_access_token(Some(AccessToken::new(
        "test_token".to_string(),
        "9999999999000".to_string(),
        "Bearer".to_string(),
    )));

    let mut client = Client::new(credentials).await?;
    client.set_version("v60.0");
    Ok(BulkApiV2::new(client))
}

#[tokio::test]
async fn test_new() -> Result<()> {
    let client = Client::new(Credentials::new()).await?;
    let api = BulkApiV2::new(client);
    assert!(api.client.instance_url().is_some());
    Ok(())
}

#[tokio::test]
async fn test_create_job() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/services/data/v60.0/jobs/ingest")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
            "id": "750xx",
            "operation": "insert",
            "object": "Account",
            "createdById": "005xx",
            "createdDate": "2024-01-01",
            "systemModstamp": "2024-01-01",
            "state": "Open",
            "concurrencyMode": "Parallel",
            "contentType": "CSV",
            "apiVersion": 60.0,
            "contentUrl": "services/data/v60.0/jobs/ingest/750xx/batches",
            "lineEnding": "LF"
            })
            .to_string(),
        )
        .create_async()
        .await;

    let mut api = create_test_bulk_api_v2(&server.url()).await.unwrap();
    let mut params = HashMap::new();
    params.insert("operation", "insert");
    params.insert("object", "Account");
    let res = api.create_job(params).await;
    assert!(res.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_upload_job_data_success() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PUT", "/services/data/v60.0/jobs/ingest/750xx/batches")
        .with_status(201)
        .create_async()
        .await;

    let mut api = create_test_bulk_api_v2(&server.url()).await.unwrap();
    let csv = b"Name\nTest Account".to_vec();
    let res = api.upload_job_data("750xx", csv).await;
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "Created");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_upload_job_data_failure() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PUT", "/services/data/v60.0/jobs/ingest/750xx/batches")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
            "message": "Invalid CSV",
            "errorCode": "INVALID_CONTENT"
            })
            .to_string(),
        )
        .create_async()
        .await;

    let mut api = create_test_bulk_api_v2(&server.url()).await.unwrap();
    let csv = b"bad data".to_vec();
    let res = api.upload_job_data("750xx", csv).await;
    assert!(res.is_err());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_all_jobs() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/services/data/v60.0/jobs/ingest/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"records": [], "done": true}).to_string())
        .create_async()
        .await;

    let mut api = create_test_bulk_api_v2(&server.url()).await.unwrap();
    let res = api.get_all_jobs().await;
    assert!(res.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_job_info() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/services/data/v60.0/jobs/ingest/750xx")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "750xx", "state": "Open"}).to_string())
        .create_async()
        .await;

    let mut api = create_test_bulk_api_v2(&server.url()).await.unwrap();
    let res = api.get_job_info("750xx").await;
    assert!(res.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_job_records() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock(
            "GET",
            "/services/data/v60.0/jobs/ingest/750xx/successfulResults",
        )
        .with_status(200)
        .with_body("sf__Id,Name\n001xx,Test")
        .create_async()
        .await;

    let mut api = create_test_bulk_api_v2(&server.url()).await.unwrap();
    let res = api.get_job_records("750xx", "successfulResults").await;
    assert!(res.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_abort_job() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PATCH", "/services/data/v60.0/jobs/ingest/750xx")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"state": "Aborted"}).to_string())
        .create_async()
        .await;

    let mut api = create_test_bulk_api_v2(&server.url()).await.unwrap();
    let res = api.abort_job("750xx").await;
    assert!(res.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_set_upload_state() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PATCH", "/services/data/v60.0/jobs/ingest/750xx")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"state": "UploadComplete"}).to_string())
        .create_async()
        .await;

    let mut api = create_test_bulk_api_v2(&server.url()).await.unwrap();
    let mut params = HashMap::new();
    params.insert("state", "UploadComplete");
    let res = api.set_upload_state("750xx", params).await;
    assert!(res.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_check_job_status() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/services/data/v60.0/jobs/ingest/750xx/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "750xx", "state": "JobComplete"}).to_string())
        .create_async()
        .await;

    let mut api = create_test_bulk_api_v2(&server.url()).await.unwrap();
    let res = api.check_job_status("750xx").await;
    assert!(res.is_ok());
    mock.assert_async().await;
}
