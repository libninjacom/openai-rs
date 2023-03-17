#![allow(unused_imports)]
use openai::OpenAiClient;
use openai::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let instruction = "your instruction";
    let model = "your model";
    let response = client
        .create_edit(instruction, model)
        .input("your input")
        .n(1)
        .temperature(1.0)
        .top_p(1.0)
        .await
        .unwrap();
    println!("{:#?}", response);
}