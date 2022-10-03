use openai2::OpenAiClient;
use openai2::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let model = "your model";
    let response = client
        .create_completion(model)
        .prompt(::serde_json::json!({}))
        .suffix("your suffix")
        .max_tokens(1)
        .temperature(1.0)
        .top_p(1.0)
        .n(1)
        .stream(true)
        .logprobs(1)
        .echo(true)
        .stop(::serde_json::json!({}))
        .presence_penalty(1.0)
        .frequency_penalty(1.0)
        .best_of(1)
        .logit_bias(::serde_json::json!({}))
        .user("your user")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
