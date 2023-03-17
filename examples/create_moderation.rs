#![allow(unused_imports)]
use openai::OpenAiClient;
use openai::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let input = serde_json::json!({});
    let response = client.create_moderation(input).model("your model").await.unwrap();
    println!("{:#?}", response);
}