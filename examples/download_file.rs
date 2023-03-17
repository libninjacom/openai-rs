#![allow(unused_imports)]
use openai::OpenAiClient;
use openai::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let file_id = "your file id";
    let response = client.download_file(file_id).await.unwrap();
    println!("{:#?}", response);
}