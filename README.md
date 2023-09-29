<div id="top"></div>

<p align="center">
    <a href="https://github.com/prabhpreet/libninja-all-of-example/stargazers">
        <img src="https://img.shields.io/github/stars/prabhpreet/libninja-all-of-example.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/prabhpreet/libninja-all-of-example/actions">
        <img src="https://img.shields.io/github/workflow/status/prabhpreet/libninja-all-of-example/ci?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/petstore">
    <img src="https://img.shields.io/crates/d/petstore?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/petstore">
    <img src="https://img.shields.io/crates/v/petstore?style=flat-square" alt="Crates.io" />
</a>

</p>

petstore client, generated from the OpenAPI spec.

# Usage

```rust
use petstore::PetstoreClient;
use petstore::model::*;
#[tokio::main]
async fn main() {
    let client = PetstoreClient::from_env();
    let response = client.list_pets().limit(1).await.unwrap();
    println!("{:#?}", response);
}
```

This example loads configuration from environment variables, specifically:



# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
petstore = "0.1.0"
```


# Documentation



* [Client Library Documentation](https://docs.rs/petstore)


You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*