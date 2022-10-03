use openai2::OpenAiClient;
use openai2::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let file_id = "your file id";
    let response = client.retrieve_file(file_id).send().await.unwrap();
    println!("{:#?}", response);
}
