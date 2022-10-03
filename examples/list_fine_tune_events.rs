use openai2::OpenAiClient;
use openai2::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let fine_tune_id = "your fine tune id";
    let response = client
        .list_fine_tune_events(fine_tune_id)
        .stream(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
