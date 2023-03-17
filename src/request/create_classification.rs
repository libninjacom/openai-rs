use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateClassificationRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
    pub examples: Option<Vec<Vec<String>>>,
    pub expand: Option<Vec<serde_json::Value>>,
    pub file: Option<String>,
    pub labels: Option<Vec<String>>,
    pub logit_bias: Option<serde_json::Value>,
    pub logprobs: Option<i64>,
    pub max_examples: Option<i64>,
    pub model: String,
    pub query: String,
    pub return_metadata: Option<bool>,
    pub return_prompt: Option<bool>,
    pub search_model: Option<String>,
    pub temperature: Option<f64>,
    pub user: Option<String>,
}
impl<'a> CreateClassificationRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CreateClassificationResponse> {
        let mut r = self.http_client.client.post("/classifications");
        if let Some(ref unwrapped) = self.examples {
            r = r.json(json!({ "examples" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.expand {
            r = r.json(json!({ "expand" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.file {
            r = r.json(json!({ "file" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.labels {
            r = r.json(json!({ "labels" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.logit_bias {
            r = r.json(json!({ "logit_bias" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.logprobs {
            r = r.json(json!({ "logprobs" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.max_examples {
            r = r.json(json!({ "max_examples" : unwrapped }));
        }
        r = r.json(json!({ "model" : self.model }));
        r = r.json(json!({ "query" : self.query }));
        if let Some(ref unwrapped) = self.return_metadata {
            r = r.json(json!({ "return_metadata" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.return_prompt {
            r = r.json(json!({ "return_prompt" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.search_model {
            r = r.json(json!({ "search_model" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.temperature {
            r = r.json(json!({ "temperature" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user {
            r = r.json(json!({ "user" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn examples(mut self, examples: &[&[&str]]) -> Self {
        self.examples = Some(examples
            .into_iter()
            .map(|s| s
                .into_iter()
                .map(|&s| s.to_owned())
                .collect()
            ).collect()
        );
        self
    }
    pub fn expand(mut self, expand: Vec<serde_json::Value>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn file(mut self, file: &str) -> Self {
        self.file = Some(file.to_owned());
        self
    }
    pub fn labels(mut self, labels: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.labels = Some(labels.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn logit_bias(mut self, logit_bias: serde_json::Value) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }
    pub fn logprobs(mut self, logprobs: i64) -> Self {
        self.logprobs = Some(logprobs);
        self
    }
    pub fn max_examples(mut self, max_examples: i64) -> Self {
        self.max_examples = Some(max_examples);
        self
    }
    pub fn return_metadata(mut self, return_metadata: bool) -> Self {
        self.return_metadata = Some(return_metadata);
        self
    }
    pub fn return_prompt(mut self, return_prompt: bool) -> Self {
        self.return_prompt = Some(return_prompt);
        self
    }
    pub fn search_model(mut self, search_model: &str) -> Self {
        self.search_model = Some(search_model.to_owned());
        self
    }
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }
    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateClassificationRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateClassificationResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}