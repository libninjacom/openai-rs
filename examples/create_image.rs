#![allow(unused_imports)]
use openai::OpenAiClient;
use openai::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let prompt = "your prompt";
    let response = client
        .create_image(prompt)
        .n(1)
        .response_format("your response format")
        .size("your size")
        .user("your user")
        .await
        .unwrap();
    println!("{:#?}", response);
}