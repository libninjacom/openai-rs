<div id="top"></div>

<p align="center">
    <a href="https://github.com/libninjacom/openai-rs/stargazers">
        <img src="https://img.shields.io/github/stars/libninjacom/openai-rs.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/libninjacom/openai-rs/actions">
        <img src="https://img.shields.io/github/actions/workflow/status/libninjacom/openai-rs/ci.yaml?
style=flat-square" alt="Build Status" />
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
use openai::OpenAiClient;
use openai::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let response = client.list_engines().await.unwrap();
    println!("{:#?}", response);
}
```

This example loads configuration from environment variables, specifically:

- `OPENAI_API_KEY` - Your OpenAI API key

# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
openai2 = "2"
```

The crate is named `openai2`, but it is imported as `openai`:

```rust
use openai;
```

# Documentation

* [Client Library Documentation](https://docs.rs/openai2)

You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*