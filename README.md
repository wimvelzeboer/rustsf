[![crate-name at crates.io](https://img.shields.io/crates/v/rustforce.svg)](https://crates.io/crates/rustforce)
[![crate-name at docs.rs](https://docs.rs/rustforce/badge.svg)](https://docs.rs/rustforce)

## RustSF

Rust Salesforce API SDK

This crate to supports the Salesforce APIs:

- Rest API, <br/>version 67.0 (latest, Summer 2026), enabled via the feature `rest-api`
- Bulk API v1
- Bulk API v2
- Streaming API

These APIs are not yet implemented, but high on the list. If you are interested in contributing, or want to speed up the development, please go to Discussions and leave a message. 
- Tooling API
- Metadata API
- CPQ API
- Heroku Platform API
- Streaming API


Forked from [rustforce](https://github.com/sile/rustforce) and from [tance77](https://github.com/tance77/rustforce/tree/modular-client-refactor), 
for the rustforce repo seemed to be abandoned. The [rustforce](https://github.com/sile/rustforce) repo also lacked some important features and had some design flows around API versioning.


## Usage

```toml
[dependencies]
rustsf = { version = "0.0.2", features = ["rest-api"] }
```

```rust
use rustsf::{Client, RestApi, Error, QueryResponse};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Account {
    id: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client_id = env::var("SFDC_CLIENT_ID").unwrap();
    let client_secret = env::var("SFDC_CLIENT_SECRET").unwrap();
    let username = env::var("SFDC_USERNAME").unwrap();
    let password = env::var("SFDC_PASSWORD").unwrap();

    let mut client = Client::new();
    client.set_client_id(&client_id);
    client.set_client_secret(&client_secret);
    client.login_with_credential(&username, &password).await?;

    let mut api = RestApi::new(client);

    let res: QueryResponse<Account> = api.query("SELECT Id, Name FROM Account").await?;
    println!("{:?}", res);

    Ok(())
}
```

### Authentication

#### Username Password Flow (OAuth2)

```rust
let mut client = Client::new();
client.set_client_id(&client_id);
client.set_client_secret(&client_secret);
client.login_with_credential(&username, &password).await?;
```

#### SOAP Login

```rust
let mut client = Client::new();
client.login_by_soap(&username, &password).await?;
```

#### Using an Existing Access Token

```rust
let mut client = Client::new();
client.set_instance_url("https://na1.salesforce.com");
client.set_access_token(token, issued_at, token_type);
```

#### SFDX AUTH URL

```rust
let mut client = Client::new();
client.login_by_sfdx_auth_url("sfdx auth url").await?;
```

#### Refresh Token

```rust
client.set_refresh_token("your_refresh_token");
client.refresh().await?;
```

### REST API

All REST API methods are accessed through `RestApi`:

[Example:](./examples/rest_api/new.rs)
```rust
let mut api = RestApi::new(client);
```


#### Query Records

```rust
let res: QueryResponse<Account> = api.query("SELECT Id, Name FROM Account").await?;
```
Examples: [Query](./examples/rest_api/query.rs), [Query All](./examples/rest_api/query_all.rs), [Query More](./examples/rest_api/query_more.rs) 

#### Find By Id
Finds a single record by its Salesforce ID. [Example:](./examples/rest_api/find_by_id.rs)
```rust
let account = api.find_by_id::<Account>("Account", "{sf_id}").await?;
```


#### CRUD Record operations

```rust
let account = Account { name: "Test Account"};

let res = api.create_sobject("Account", account).await?;
let res = api.create("Account", vec![account]).await?;
```

Examples:
- Create a single record [Simple example](./examples/rest_api/create.rs) or [Alternative example](./examples/rest_api/create_alt.rs)
- [Create](./examples/rest_api/create.rs)

#### Update Record

```rust
api.update("Account", "{sobject_id}", params).await?;
```

#### Upsert Record

```rust
let res = api.upsert("Account", "{external_key_name}", "{external_key}", params).await?;
println!("{:?}", res.status()); // 200 = updated, 201 = created
```

#### Delete Record

```rust
api.destroy("Account", "{sobject_id}").await?;
```

#### Describe Global

```rust
use rustsf::rest_api::responses::DescribeGlobalResponse;

let res: DescribeGlobalResponse = api.describe_global().await?;
```

#### Describe SObject

```rust
use rustforce::DescribeResponse;

let res: DescribeResponse = api.describe("Account").await?;
```

#### Versions

```rust
use rustforce::VersionResponse;

let versions: Vec<VersionResponse> = api.versions().await?;
```

#### Search (SOSL)

```rust
use rustforce::SearchResponse;

let res: SearchResponse = api.search_sosl("FIND {Rust}").await?;
```

### Bulk API v1

```rust
use rustforce::BulkApi;

let mut bulk = BulkApi::new(client);
let res = bulk.create_job(params).await?;
```

### Bulk API v2

```rust
use rustforce::BulkApiV2;

let mut bulk = BulkApiV2::new(client);
let res = bulk.create_job(params).await?;
```
