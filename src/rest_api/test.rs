use mockito::Server;
use crate::client::client::{Client};
use crate::errors::Error;
use super::*;

use crate as rustsf;
use serde_json::json;
use crate::DefSObject;
use crate::rest_api::responses::query_response::QueryResponse;
use crate::primary_types::{SObjectOwner};

fn create_test_rest_api(server_url: &str) -> RestApi {
    let mut client = Client::new();
    client.set_instance_url(server_url);
    client.set_access_token(
        "test_token".to_string(),
        "9999999999000".to_string(),
        "Bearer".to_string(),
    );
    client.set_version("v60.0");
    RestApi::new(client)
}

#[tokio::test]
async fn test_new() {
    let client = Client::new();
    let api = RestApi::new(client);
    assert!(api.client.instance_url.is_none());
}

#[tokio::test]
async fn test_base_path() {
    let mut client = Client::new();
    client.set_instance_url("https://na1.salesforce.com");
    client.set_version("v60.0");
    let api = RestApi::new(client);
    assert_eq!(
        api.client.base_version_path().unwrap(),
        "https://na1.salesforce.com/services/data/v60.0"
    );
}

#[tokio::test]
async fn test_base_path_not_logged_in() {
    let client = Client::new();
    let api = RestApi::new(client);
    let result = api.client.base_version_path();
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::NotLoggedIn => {}
        e => panic!("Expected NotLoggedIn, got {:?}", e),
    }
}

#[DefSObject(sobject_type = "Account", fields="name,owner")]
struct Account { }

impl Account {
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_str())
    }
    pub fn set_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}

#[tokio::test]
async fn test_query() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/services/data/v60.0/query/")
        .match_query(mockito::Matcher::UrlEncoded(
            "q".into(),
            "SELECT Id FROM Account".into(),
        ))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                    "totalSize": 1,
                    "done": true,
                    "records": []
                })
                .to_string(),
        )
        .create_async()
        .await;

    let mut api = create_test_rest_api(&server.url());
    let res : QueryResponse<Account> = api.query("SELECT Id FROM Account").await.unwrap();
    assert_eq!(res.total_size, 1);
    assert_eq!(res.done, true);
    assert!(res.records.is_empty());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_query_all() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/services/data/v60.0/queryAll/")
        .match_query(mockito::Matcher::UrlEncoded(
            "q".into(),
            "SELECT Id FROM Account".into(),
        ))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                    "totalSize": 0,
                    "done": true,
                    "records": []
                })
                .to_string(),
        )
        .create_async()
        .await;

    let mut api = create_test_rest_api(&server.url());
    let res = api.query_all::<Account>("SELECT Id FROM Account").await.unwrap();
    assert_eq!(res.done, true);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_query_more() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/services/data/v60.0/query/01gxx-2000")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                    "totalSize": 5000,
                    "done": true,
                    "records": []
                })
                .to_string(),
        )
        .create_async()
        .await;

    let mut api = create_test_rest_api(&server.url());
    let res = api
        .query_more::<Account>("services/data/v60.0/query/01gxx-2000")
        .await
        .unwrap();
    assert_eq!(res.total_size, 5000);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_search_sosl() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/services/data/v60.0/search/")
        .match_query(mockito::Matcher::UrlEncoded(
            "q".into(),
            "FIND {test}".into(),
        ))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"searchRecords": []}).to_string())
        .create_async()
        .await;

    let mut api = create_test_rest_api(&server.url());
    let res = api.search_sosl("FIND {test}").await.unwrap();
    assert!(res.search_records.is_empty());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_versions() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/services/data/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!([{
                    "label": "Spring '24",
                    "url": "/services/data/v60.0",
                    "version": "60.0"
                }])
                .to_string(),
        )
        .create_async()
        .await;

    let mut api = create_test_rest_api(&server.url());
    let res = api.api_versions().await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res.iter().nth(0).unwrap().version, "60.0");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_versions_not_logged_in() {
    let client = Client::new();
    let mut api = RestApi::new(client);
    let res = api.api_versions().await;
    assert!(res.is_err());
    match res.unwrap_err() {
        Error::NotLoggedIn => {}
        e => panic!("Expected NotLoggedIn, got {:?}", e),
    }
}

#[tokio::test]
async fn test_find_by_id() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/services/data/v60.0/sobjects/Account/001xx000003DGbX")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({"Id": "001xx000003DGbX", "Name": "Acme"}).to_string())
        .create_async()
        .await;

    let mut api = create_test_rest_api(&server.url());
    let account = api.fetch_by_id::<Account>("Account", "001xx000003DGbX").await.unwrap();
    assert_eq!(account.id(), Some("001xx000003DGbX"));
    assert_eq!(account.get_name(), Some("Acme"));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_create() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/services/data/v60.0/sobjects/Account")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "001xx000003DGbX", "success": true}).to_string())
        .create_async()
        .await;

    let mut api = create_test_rest_api(&server.url());
    let account = Account::new().set_name("Test Account".to_string());

    let account = api.create_sobject(account).await.unwrap();
    assert_eq!(account.id(), Some("001xx000003DGbX"));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_update() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("PATCH", "/services/data/v60.0/sobjects/Account/001xx")
        .with_status(204)
        .create_async()
        .await;

    let mut api = create_test_rest_api(&server.url());
    let mut params = std::collections::HashMap::new();
    params.insert("Name", "Updated");
    let res = api.update_sobject("Account", "001xx", params).await;
    assert!(res.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_upsert() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock(
            "PATCH",
            "/services/data/v60.0/sobjects/Account/ExternalId__c/ext123",
        )
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(json!({"id": "001xx", "success": true}).to_string())
        .create_async()
        .await;

    let mut api = create_test_rest_api(&server.url());
    let mut params = std::collections::HashMap::new();
    params.insert("Name", "Upserted");
    let res = api
        .upsert_sobject("Account", "ExternalId__c", "ext123", params)
        .await;
    assert!(res.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_destroy() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("DELETE", "/services/data/v60.0/sobjects/Account/001xx")
        .with_status(204)
        .create_async()
        .await;

    let mut api = create_test_rest_api(&server.url());
    let res = api.delete_sobject("Account", "001xx").await;
    assert!(res.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_describe_global() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/services/data/v60.0/sobjects")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                    "encoding": "UTF-8",
                    "maxBatchSize": 200,
                    "sobjects": []
                })
                .to_string(),
        )
        .create_async()
        .await;

    let mut api = create_test_rest_api(&server.url());
    let res = api.describe_global().await.unwrap();
    assert_eq!(res.encoding, "UTF-8");
    assert_eq!(res.max_batch_size, 200);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_describe() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/services/data/v60.0/sobjects/Account/describe")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                    "name": "Account",
                    "childRelationships": [],
                    "label": "Account",
                    "labelPlural": "Accounts",
                    "createable": true,
                    "urls": {
                        "compactLayouts" : "value",
                        "rowTemplate" : "value",
                        "approvalLayouts" : "value",
                        "uiDetailTemplate" : "value",
                        "uiEditTemplate" : "value",
                        "defaultValues" : "value",
                        "listviews" : "value",
                        "describe" : "value",
                        "uiNewRecord" : "value",
                        "quickActions" : "value",
                        "layouts" : "value",
                        "sobject" : "value",
                    },
                })
                .to_string(),
        )
        .create_async()
        .await;

    let mut api = create_test_rest_api(&server.url());
    let res = api.describe_sobject("Account").await.unwrap();
    assert_eq!(res.name, "Account");
    assert_eq!(res.createable, true);
    mock.assert_async().await;
}