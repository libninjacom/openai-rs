use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateFineTuneRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
    pub batch_size: Option<i64>,
    pub classification_betas: Option<Vec<f64>>,
    pub classification_n_classes: Option<i64>,
    pub classification_positive_class: Option<String>,
    pub compute_classification_metrics: Option<bool>,
    pub learning_rate_multiplier: Option<f64>,
    pub model: Option<String>,
    pub n_epochs: Option<i64>,
    pub prompt_loss_weight: Option<f64>,
    pub suffix: Option<String>,
    pub training_file: String,
    pub validation_file: Option<String>,
}
impl<'a> CreateFineTuneRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<FineTune> {
        let mut r = self.http_client.client.post("/fine-tunes");
        if let Some(ref unwrapped) = self.batch_size {
            r = r.json(json!({ "batch_size" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.classification_betas {
            r = r.json(json!({ "classification_betas" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.classification_n_classes {
            r = r.json(json!({ "classification_n_classes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.classification_positive_class {
            r = r.json(json!({ "classification_positive_class" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.compute_classification_metrics {
            r = r.json(json!({ "compute_classification_metrics" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.learning_rate_multiplier {
            r = r.json(json!({ "learning_rate_multiplier" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.model {
            r = r.json(json!({ "model" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.n_epochs {
            r = r.json(json!({ "n_epochs" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.prompt_loss_weight {
            r = r.json(json!({ "prompt_loss_weight" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.suffix {
            r = r.json(json!({ "suffix" : unwrapped }));
        }
        r = r.json(json!({ "training_file" : self.training_file }));
        if let Some(ref unwrapped) = self.validation_file {
            r = r.json(json!({ "validation_file" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn batch_size(mut self, batch_size: i64) -> Self {
        self.batch_size = Some(batch_size);
        self
    }
    pub fn classification_betas(mut self, classification_betas: Vec<f64>) -> Self {
        self.classification_betas = Some(classification_betas);
        self
    }
    pub fn classification_n_classes(mut self, classification_n_classes: i64) -> Self {
        self.classification_n_classes = Some(classification_n_classes);
        self
    }
    pub fn classification_positive_class(
        mut self,
        classification_positive_class: &str,
    ) -> Self {
        self
            .classification_positive_class = Some(
            classification_positive_class.to_owned(),
        );
        self
    }
    pub fn compute_classification_metrics(
        mut self,
        compute_classification_metrics: bool,
    ) -> Self {
        self.compute_classification_metrics = Some(compute_classification_metrics);
        self
    }
    pub fn learning_rate_multiplier(mut self, learning_rate_multiplier: f64) -> Self {
        self.learning_rate_multiplier = Some(learning_rate_multiplier);
        self
    }
    pub fn model(mut self, model: &str) -> Self {
        self.model = Some(model.to_owned());
        self
    }
    pub fn n_epochs(mut self, n_epochs: i64) -> Self {
        self.n_epochs = Some(n_epochs);
        self
    }
    pub fn prompt_loss_weight(mut self, prompt_loss_weight: f64) -> Self {
        self.prompt_loss_weight = Some(prompt_loss_weight);
        self
    }
    pub fn suffix(mut self, suffix: &str) -> Self {
        self.suffix = Some(suffix.to_owned());
        self
    }
    pub fn validation_file(mut self, validation_file: &str) -> Self {
        self.validation_file = Some(validation_file.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateFineTuneRequest<'a> {
    type Output = httpclient::InMemoryResult<FineTune>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}