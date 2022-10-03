<div id="top"></div>

<p align="center">
    <a href="https://github.com/libninjacom/openai-rs/graphs/contributors">
        <img src="https://img.shields.io/github/contributors/libninjacom/openai-rs.svg?style=flat-square" alt="GitHub Contributors" />
    </a>
    <a href="https://github.com/libninjacom/openai-rs/stargazers">
        <img src="https://img.shields.io/github/stars/libninjacom/openai-rs.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/libninjacom/openai-rs/actions">
        <img src="https://img.shields.io/github/workflow/status/libninjacom/openai-rs/test?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/openai2">
    <img src="https://img.shields.io/crates/d/openai2?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/openai2">
    <img src="https://img.shields.io/crates/v/openai2?style=flat-square" alt="Crates.io" />
</a>

</p>

OpenAi client, generated from the OpenAPI spec.

# Usage

```rust
use openai2::OpenAiClient;
use openai2::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let response = client.list_engines().send().await.unwrap();
    println!("{:#?}", response);
}

```

This example loads configuration from environment variables, specifically:



# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
openai2 = "2.0.0"
```


# Documentation



* [Client Library Documentation](https://docs.rs/openai2)


You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*