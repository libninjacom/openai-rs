#![allow(unused_imports)]
use openai::OpenAiClient;
use openai::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let model = "your model";
    let query = "your query";
    let response = client
        .create_classification(model, query)
        .examples(&[&["your examples"]])
        .expand(vec![serde_json::json!({})])
        .file("your file")
        .labels(&["your labels"])
        .logit_bias(serde_json::json!({}))
        .logprobs(1)
        .max_examples(1)
        .return_metadata(true)
        .return_prompt(true)
        .search_model("your search model")
        .temperature(1.0)
        .user("your user")
        .await
        .unwrap();
    println!("{:#?}", response);
}