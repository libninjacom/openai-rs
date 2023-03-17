#![allow(unused_imports)]
use openai::OpenAiClient;
use openai::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let model = "your model";
    let response = client
        .create_completion(model)
        .best_of(1)
        .echo(true)
        .frequency_penalty(1.0)
        .logit_bias(serde_json::json!({}))
        .logprobs(1)
        .max_tokens(1)
        .n(1)
        .presence_penalty(1.0)
        .prompt(serde_json::json!({}))
        .stop(serde_json::json!({}))
        .stream(true)
        .suffix("your suffix")
        .temperature(1.0)
        .top_p(1.0)
        .user("your user")
        .await
        .unwrap();
    println!("{:#?}", response);
}