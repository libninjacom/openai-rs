use openai2::OpenAiClient;
use openai2::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let response = client.create_file().send().await.unwrap();
    println!("{:#?}", response);
}
