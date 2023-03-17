#![allow(unused_imports)]
use openai::OpenAiClient;
use openai::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let training_file = "your training file";
    let response = client
        .create_fine_tune(training_file)
        .batch_size(1)
        .classification_betas(vec![1.0])
        .classification_n_classes(1)
        .classification_positive_class("your classification positive class")
        .compute_classification_metrics(true)
        .learning_rate_multiplier(1.0)
        .model("your model")
        .n_epochs(1)
        .prompt_loss_weight(1.0)
        .suffix("your suffix")
        .validation_file("your validation file")
        .await
        .unwrap();
    println!("{:#?}", response);
}