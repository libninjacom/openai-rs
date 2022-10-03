use openai2::OpenAiClient;
use openai2::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let model = "your model";
    let instruction = "your instruction";
    let response = client
        .create_edit(model, instruction)
        .input("your input")
        .n(1)
        .temperature(1.0)
        .top_p(1.0)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
