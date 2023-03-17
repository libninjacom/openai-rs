#![allow(unused_imports)]
use openai::OpenAiClient;
use openai::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let input = serde_json::json!({});
    let model = "your model";
    let response = client
        .create_embedding(input, model)
        .user("your user")
        .await
        .unwrap();
    println!("{:#?}", response);
}