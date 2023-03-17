use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateAnswerRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
    pub documents: Option<Vec<String>>,
    pub examples: Vec<Vec<String>>,
    pub examples_context: String,
    pub expand: Option<Vec<serde_json::Value>>,
    pub file: Option<String>,
    pub logit_bias: Option<serde_json::Value>,
    pub logprobs: Option<i64>,
    pub max_rerank: Option<i64>,
    pub max_tokens: Option<i64>,
    pub model: String,
    pub n: Option<i64>,
    pub question: String,
    pub return_metadata: Option<bool>,
    pub return_prompt: Option<bool>,
    pub search_model: Option<String>,
    pub stop: Option<serde_json::Value>,
    pub temperature: Option<f64>,
    pub user: Option<String>,
}
impl<'a> CreateAnswerRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateAnswerResponse> {
        let mut r = self.http_client.client.post("/answers");
        if let Some(ref unwrapped) = self.documents {
            r = r.json(json!({ "documents" : unwrapped }));
        }
        r = r.json(json!({ "examples" : self.examples }));
        r = r.json(json!({ "examples_context" : self.examples_context }));
        if let Some(ref unwrapped) = self.expand {
            r = r.json(json!({ "expand" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.file {
            r = r.json(json!({ "file" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.logit_bias {
            r = r.json(json!({ "logit_bias" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.logprobs {
            r = r.json(json!({ "logprobs" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.max_rerank {
            r = r.json(json!({ "max_rerank" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.max_tokens {
            r = r.json(json!({ "max_tokens" : unwrapped }));
        }
        r = r.json(json!({ "model" : self.model }));
        if let Some(ref unwrapped) = self.n {
            r = r.json(json!({ "n" : unwrapped }));
        }
        r = r.json(json!({ "question" : self.question }));
        if let Some(ref unwrapped) = self.return_metadata {
            r = r.json(json!({ "return_metadata" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.return_prompt {
            r = r.json(json!({ "return_prompt" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.search_model {
            r = r.json(json!({ "search_model" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.stop {
            r = r.json(json!({ "stop" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.temperature {
            r = r.json(json!({ "temperature" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user {
            r = r.json(json!({ "user" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn documents(
        mut self,
        documents: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .documents = Some(
            documents.into_iter().map(|s| s.as_ref().to_owned()).collect(),
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
    pub fn logit_bias(mut self, logit_bias: serde_json::Value) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }
    pub fn logprobs(mut self, logprobs: i64) -> Self {
        self.logprobs = Some(logprobs);
        self
    }
    pub fn max_rerank(mut self, max_rerank: i64) -> Self {
        self.max_rerank = Some(max_rerank);
        self
    }
    pub fn max_tokens(mut self, max_tokens: i64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }
    pub fn n(mut self, n: i64) -> Self {
        self.n = Some(n);
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
    pub fn stop(mut self, stop: serde_json::Value) -> Self {
        self.stop = Some(stop);
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
pub struct CreateAnswerRequired<'a> {
    pub examples: &'a [&'a [&'a str]],
    pub examples_context: &'a str,
    pub model: &'a str,
    pub question: &'a str,
}
impl<'a> CreateAnswerRequired<'a> {}
impl<'a> ::std::future::IntoFuture for CreateAnswerRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateAnswerResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}