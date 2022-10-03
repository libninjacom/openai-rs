use openai2::OpenAiClient;
use openai2::model::*;
#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let training_file = "your training file";
    let response = client
        .create_fine_tune(training_file)
        .validation_file("your validation file")
        .model("your model")
        .n_epochs(1)
        .batch_size(1)
        .learning_rate_multiplier(1.0)
        .prompt_loss_weight(1.0)
        .compute_classification_metrics(true)
        .classification_n_classes(1)
        .classification_positive_class("your classification positive class")
        .classification_betas(&[1.0])
        .suffix("your suffix")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
