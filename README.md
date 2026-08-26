[![crate-name at crates.io](https://img.shields.io/crates/v/rustforce.svg)](https://crates.io/crates/rustforce)
[![crate-name at docs.rs](https://docs.rs/rustforce/badge.svg)](https://docs.rs/rustforce)

## RustSF

Thé most comprehensive Salesforce SDK for Rust, that supports most of the primary Salesforce APIs.
It contains many examples and has very detailed documententation. 

The currently supported Salesforce APIs with the latest version 67.0 (Summer 2026) are:

- Rest API, enabled via the feature `rest-api`
- Tooling API, enabled via the feature `tooling-api`
- Metadata API, enabled via the feature `metadata-api`

_It also supports the following APIs, but they still need a bit of work:_

- Bulk API v1
- Bulk API v2
- Streaming API

And we have a number of APIs which are high on the wish list. 
If you are interested in contributing, please start a discussion with your suggestions. 

- CPQ API
- Heroku Platform API
- Streaming API


#### Notes
This repo was forked from [rustforce](https://github.com/sile/rustforce) and a PR from [tance77](https://github.com/tance77/rustforce/tree/modular-client-refactor), 
for the rustforce repo seemed to be abandoned. The [rustforce](https://github.com/sile/rustforce) repo also lacked some important
features and had some design flows around API versioning.


## Usage

```toml
[dependencies]
rustsf = { version = "0.0.2", features = ["rest-api", "tooling-api", "metadata-api"] }
```

```rust
use rustsf::{Client, RestApi, Credentials, QueryResponse};
use serde::Deserialize;
use std::env;
use anyhow::Result;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Account {
    id: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut credentials = Credentials::new();
    credentials.set_client_id(env::var("SFDC_CLIENT_ID").unwrap());
    credentials.set_client_secret(env::var("SFDC_CLIENT_SECRET").unwrap());
    credentials.set_username(env::var("SFDC_USERNAME").unwrap());
    credentials.set_password(env::var("SFDC_PASSWORD").unwrap());

    let mut client = Client::new(credentials).await?;
    let mut api = RestApi::new(client);

    let res: QueryResponse<Account> = api.query("SELECT Id, Name FROM Account").await?;
    println!("{:?}", res);

    Ok(())
}
```

### Authentication

#### Username Password Flow (OAuth2)

```rust
let mut credentials = Credentials::new();
credentials.set_client_id(env::var("SFDC_CLIENT_ID").unwrap());
credentials.set_client_secret(env::var("SFDC_CLIENT_SECRET").unwrap());
credentials.set_username(env::var("SFDC_USERNAME").unwrap());
credentials.set_password(env::var("SFDC_PASSWORD").unwrap());

let mut client = Client::new(credentials).await?;
```

#### Using an Existing Access Token

```rust
let mut credentials = Credentials::new();
credentials.set_access_token(Some(AccessToken {
    value,
    issued_at,
    token_type,
}));
credentials.set_instance_url("https://na1.salesforce.com");
let mut client = Client::new(credentials).await?;
```

#### SFDX AUTH URL

```rust
let mut client = Client::new(AuthUrl::new("...SFDX_AUTH_URL...")).await?;
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
