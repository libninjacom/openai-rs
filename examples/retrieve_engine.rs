use openai2::OpenAiClient;
use openai2::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let engine_id = "your engine id";
    let response = client.retrieve_engine(engine_id).send().await.unwrap();
    println!("{:#?}", response);
}
