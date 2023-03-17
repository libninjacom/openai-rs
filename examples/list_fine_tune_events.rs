#![allow(unused_imports)]
use openai::OpenAiClient;
use openai::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let fine_tune_id = "your fine tune id";
    let response = client
        .list_fine_tune_events(fine_tune_id)
        .stream(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}