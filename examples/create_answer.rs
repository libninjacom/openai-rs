#![allow(unused_imports)]
use openai::OpenAiClient;
use openai::model::*;
use openai::request::CreateAnswerRequired;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let args = CreateAnswerRequired {
        examples: &[&["your examples"]],
        examples_context: "your examples context",
        model: "your model",
        question: "your question",
    };
    let response = client
        .create_answer(args)
        .documents(&["your documents"])
        .expand(vec![serde_json::json!({})])
        .file("your file")
        .logit_bias(serde_json::json!({}))
        .logprobs(1)
        .max_rerank(1)
        .max_tokens(1)
        .n(1)
        .return_metadata(true)
        .return_prompt(true)
        .search_model("your search model")
        .stop(serde_json::json!({}))
        .temperature(1.0)
        .user("your user")
        .await
        .unwrap();
    println!("{:#?}", response);
}