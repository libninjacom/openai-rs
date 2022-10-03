use openai2::OpenAiClient;
use openai2::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let model = "your model";
    let query = "your query";
    let response = client
        .create_classification(model, query)
        .examples(vec![vec!["your examples".to_owned()]])
        .file("your file")
        .labels(&["your labels"])
        .search_model("your search model")
        .temperature(1.0)
        .logprobs(1)
        .max_examples(1)
        .logit_bias(::serde_json::json!({}))
        .return_prompt(true)
        .return_metadata(true)
        .expand(vec![::serde_json::json!({})])
        .user("your user")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
