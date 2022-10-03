use openai2::OpenAiClient;
use openai2::model::*;
use openai2::request::CreateAnswerRequired;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let args = CreateAnswerRequired {
        examples_context: "your examples context",
        examples: vec![vec!["your examples".to_owned()]],
        model: "your model",
        question: "your question",
    };
    let response = client
        .create_answer(args)
        .documents(&["your documents"])
        .file("your file")
        .search_model("your search model")
        .max_rerank(1)
        .temperature(1.0)
        .logprobs(1)
        .max_tokens(1)
        .stop(::serde_json::json!({}))
        .n(1)
        .logit_bias(::serde_json::json!({}))
        .return_metadata(true)
        .return_prompt(true)
        .expand(vec![::serde_json::json!({})])
        .user("your user")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
