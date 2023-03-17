#![allow(unused_imports)]
use openai::OpenAiClient;
use openai::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let fine_tune_id = "your fine tune id";
    let response = client.retrieve_fine_tune(fine_tune_id).await.unwrap();
    println!("{:#?}", response);
}