use openai2::OpenAiClient;
use openai2::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let response = client.list_files().send().await.unwrap();
    println!("{:#?}", response);
}
