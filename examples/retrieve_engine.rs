#![allow(unused_imports)]
use openai::OpenAiClient;
use openai::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let engine_id = "your engine id";
    let response = client.retrieve_engine(engine_id).await.unwrap();
    println!("{:#?}", response);
}