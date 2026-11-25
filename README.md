[![crate-name at crates.io](https://img.shields.io/crates/v/rustsf.svg)](https://crates.io/crates/rustsf)
[![crate-name at docs.rs](https://docs.rs/rustsf/badge.svg)](https://docs.rs/rustsf)

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
rustsf = { version = "0.1.5", features = ["rest-api", "tooling-api", "metadata-api"] }
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

    let mut session = Client::new(credentials).await?.connect().await?;
    let mut api = RestApi::new(session);

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
let session = client.connect().await?;
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
let session = client.connect().await?;
```

#### SFDX AUTH URL

```rust
let mut client = Client::new(AuthUrl::new("...SFDX_AUTH_URL...")).await?;
let session = client.connect().await?;
```

### REST API

All REST API methods are accessed through `RestApi`:

**Examples:**
```rust
let mut api = RestApi::new(client);

// Query records
let accounts = api.query::<Account>("SELECT Id, Name FROM Account")
println!("{:?}", accounts);

// Find by Id
let account = api.find_by_id::<Account>("Account", "{sf_id}").await?;
println!("{:?}", account);

// Create a new records
let account = Account { name: "Test Account"};
let res = api.create_sobject("Account", account).await?; // Single
let res = api.create("Account", vec![account]).await?; // Multiple

// Update Record
api.update("Account", "0012K00001drfGYQAY", json!({"Name": "hello rust updated"})).await?;
  
// Delete Record
api.delete_sobject("Account", "0011t00001FfDoaAAF").await?;
```


## Structs
Some useful structs:
- Manifest, to generate package.xml files. 