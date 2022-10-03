use openai2::OpenAiClient;
use openai2::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let engine_id = "your engine id";
    let query = "your query";
    let response = client
        .create_search(engine_id, query)
        .documents(&["your documents"])
        .file("your file")
        .max_rerank(1)
        .return_metadata(true)
        .user("your user")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
