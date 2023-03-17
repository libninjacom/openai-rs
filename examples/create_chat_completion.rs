#![allow(unused_imports)]
use openai::OpenAiClient;
use openai::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let messages = vec![
        ChatCompletionRequestMessage { content : "your content".to_owned(), name :
        Some("your name".to_owned()), role : "your role".to_owned() }
    ];
    let model = "your model";
    let response = client
        .create_chat_completion(messages, model)
        .frequency_penalty(1.0)
        .logit_bias(serde_json::json!({}))
        .max_tokens(1)
        .n(1)
        .presence_penalty(1.0)
        .stop(serde_json::json!({}))
        .stream(true)
        .temperature(1.0)
        .top_p(1.0)
        .user("your user")
        .await
        .unwrap();
    println!("{:#?}", response);
}