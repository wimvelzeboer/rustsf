use crate::client::client::Client;
use crate::errors::Error;
use crate::responses::error_response::ErrorResponse;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::responses::create_response::CreateResponse;
use crate::responses::describe_global_response::DescribeGlobalResponse;
use crate::responses::describe_response::DescribeResponse;
use crate::responses::query_response::QueryResponse;
use crate::responses::search_response::SearchResponse;
use crate::responses::version_response::VersionResponse;

#[derive(Default)]
pub struct RestApi {
    pub(crate) client: Client,
}

async fn handle_json_response<T: DeserializeOwned>(response: Response) -> Result<T, Error> {    if response.status().is_success() {
        Ok(response.json().await?)
    } else {
        let errors: Vec<ErrorResponse> = response.json().await?;
        Err(Error::ErrorResponses(errors))
    }
}

async fn handle_empty_response(response: Response) -> Result<(), Error> {
    if response.status().is_success() {
        Ok(())
    } else {
        let errors: Vec<ErrorResponse> = response.json().await?;
        Err(Error::ErrorResponses(errors))
    }
}

impl RestApi {
    pub fn new(client: Client) -> Self {
        RestApi { client }
    }

    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_query.htm>
    pub async fn query<T: DeserializeOwned>(&mut self, query: &str) -> Result<QueryResponse<T>, Error> {
        let query_url = format!("{}/query/", self.client.base_path()?);
        let params = vec![("q".to_string(), query.to_string())];
        let response = self.client.get(query_url, params).await?;
        handle_json_response(response).await
    }

    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_queryall.htm>
    pub async fn query_all<T: DeserializeOwned>(&mut self, query: &str) -> Result<QueryResponse<T>, Error> {
        let query_url = format!("{}/queryAll/", self.client.base_path()?);
        let params = vec![("q".to_string(), query.to_string())];
        let response = self.client.get(query_url, params).await?;
        handle_json_response(response).await
    }

    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_queryall_more_results.htm>
    pub async fn query_more<T: DeserializeOwned>(&mut self, next_records_url: &str) -> Result<QueryResponse<T>, Error> {
        let instance_url = self
            .client
            .instance_url
            .as_ref()
            .ok_or(Error::NotLoggedIn)?;
        let query_url = format!("{}/{}", instance_url, next_records_url);
        let response = self.client.get(query_url, vec![]).await?;
        handle_json_response(response).await
    }

    /// Salesforce Object Search Language (SOSL)
    ///
    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_search.htm>
    pub async fn search_sosl(&mut self, query: &str) -> Result<SearchResponse, Error> {
        let query_url = format!("{}/search/", self.client.base_path()?);
        let params = vec![("q".to_string(), query.to_string())];
        let response = self.client.get(query_url, params).await?;
        handle_json_response(response).await
    }

    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_versions.htm>
    pub async fn versions(&mut self) -> Result<Vec<VersionResponse>, Error> {
        let instance_url = match self.client.instance_url.as_ref() {
            Some(url) => url,
            None => return Err(Error::NotLoggedIn),
        };
        let versions_url = format!("{}/services/data/", instance_url);
        let response = self.client.get(versions_url, vec![]).await?;
        handle_json_response(response).await
    }

    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_retrieve_get.htm>
    pub async fn find_by_id<T :DeserializeOwned>(
        &mut self,
        sobject_name: &str,
        id: &str,
    ) -> Result<T, Error> {
        let resource_url = format!("{}/sobjects/{}/{}", self.client.base_path()?, sobject_name, id);
        let response = self.client.get(resource_url, vec![]).await?;
        handle_json_response(response).await
    }

    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_basic_info_post.htm>
    pub async fn create<T: Serialize>(
        &mut self,
        object_name: &str,
        params: T,
    ) -> Result<CreateResponse, Error> {
        let resource_url = format!("{}/sobjects/{}", self.client.base_path()?, object_name);
        let response = self.client.post(resource_url, params, vec![]).await?;
        handle_json_response(response).await
    }

    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_retrieve_get.htm>
    pub async fn update<T: Serialize>(
        &mut self,
        object_name: &str,
        id: &str,
        params: T,
    ) -> Result<(), Error> {
        let resource_url = format!("{}/sobjects/{}/{}", self.client.base_path()?, object_name, id);
        let response = self.client.patch(resource_url, params).await?;
        handle_empty_response(response).await
    }

    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_upsert_patch.htm>
    pub async fn upsert<T: Serialize>(
        &mut self,
        sobject_name: &str,
        key_name: &str,
        key: &str,
        params: T,
    ) -> Result<Response, Error> {
        let resource_url = format!(
            "{}/sobjects/{}/{}/{}",
            self.client.base_path()?,
            sobject_name,
            key_name,
            key
        );
        self.client.patch(resource_url, params).await
    }

    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_retrieve_delete.htm>
    pub async fn destroy(&mut self, sobject_name: &str, id: &str) -> Result<(), Error> {
        let resource_url = format!("{}/sobjects/{}/{}", self.client.base_path()?, sobject_name, id);
        let response = self.client.delete(resource_url).await?;
        handle_empty_response(response).await
    }

    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_describeGlobal.htm>
    pub async fn describe_global(&mut self) -> Result<DescribeGlobalResponse, Error> {
        let resource_url = format!("{}/sobjects", self.client.base_path()?);
        let response = self.client.get(resource_url, vec![]).await?;
        handle_json_response(response).await
    }

    /// <https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_describe.htm>
    pub async fn describe(&mut self, object_name: &str) -> Result<DescribeResponse, Error> {
        let resource_url = format!("{}/sobjects/{}/describe", self.client.base_path()?, object_name);
        let response = self.client.get(resource_url, vec![]).await?;
        handle_json_response(response).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use serde_json::json;

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

    #[test]
    fn test_new() {
        let client = Client::new();
        let api = RestApi::new(client);
        assert!(api.client.instance_url.is_none());
    }

    #[test]
    fn test_base_path() {
        let mut client = Client::new();
        client.set_instance_url("https://na1.salesforce.com");
        client.set_version("v60.0");
        let api = RestApi::new(client);
        assert_eq!(
            api.client.base_path().unwrap(),
            "https://na1.salesforce.com/services/data/v60.0"
        );
    }

    #[test]
    fn test_base_path_not_logged_in() {
        let client = Client::new();
        let api = RestApi::new(client);
        let result = api.client.base_path();
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::NotLoggedIn => {}
            e => panic!("Expected NotLoggedIn, got {:?}", e),
        }
    }

    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct Account {
        id: String,
        name: String,
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
        let res = api.versions().await.unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res.iter().nth(0).unwrap().version, "60.0");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_versions_not_logged_in() {
        let client = Client::new();
        let mut api = RestApi::new(client);
        let res = api.versions().await;
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
        let res = api.find_by_id::<Account>("Account", "001xx000003DGbX").await.unwrap();
        assert_eq!(res.id, "001xx000003DGbX");
        assert_eq!(res.name, "Acme");
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
        let mut params = std::collections::HashMap::new();
        params.insert("Name", "Test Account");
        let res = api.create("Account", params).await.unwrap();
        assert_eq!(res.id, "001xx000003DGbX");
        assert_eq!(res.success, true);
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
        let res = api.update("Account", "001xx", params).await;
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
            .upsert("Account", "ExternalId__c", "ext123", params)
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
        let res = api.destroy("Account", "001xx").await;
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
        let res = api.describe("Account").await.unwrap();
        assert_eq!(res.name, "Account");
        assert_eq!(res.createable, true);
        mock.assert_async().await;
    }
}
