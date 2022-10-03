use openai2::OpenAiClient;
use openai2::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let model = "your model";
    let response = client.delete_model(model).send().await.unwrap();
    println!("{:#?}", response);
}
