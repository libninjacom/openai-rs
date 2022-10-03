use openai2::OpenAiClient;
use openai2::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let model = "your model";
    let input = ::serde_json::json!({});
    let response = client
        .create_embedding(model, input)
        .user("your user")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
