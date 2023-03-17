use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateCompletionRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
    pub best_of: Option<i64>,
    pub echo: Option<bool>,
    pub frequency_penalty: Option<f64>,
    pub logit_bias: Option<serde_json::Value>,
    pub logprobs: Option<i64>,
    pub max_tokens: Option<i64>,
    pub model: String,
    pub n: Option<i64>,
    pub presence_penalty: Option<f64>,
    pub prompt: Option<serde_json::Value>,
    pub stop: Option<serde_json::Value>,
    pub stream: Option<bool>,
    pub suffix: Option<String>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub user: Option<String>,
}
impl<'a> CreateCompletionRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateCompletionResponse> {
        let mut r = self.http_client.client.post("/completions");
        if let Some(ref unwrapped) = self.best_of {
            r = r.json(json!({ "best_of" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.echo {
            r = r.json(json!({ "echo" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.frequency_penalty {
            r = r.json(json!({ "frequency_penalty" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.logit_bias {
            r = r.json(json!({ "logit_bias" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.logprobs {
            r = r.json(json!({ "logprobs" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.max_tokens {
            r = r.json(json!({ "max_tokens" : unwrapped }));
        }
        r = r.json(json!({ "model" : self.model }));
        if let Some(ref unwrapped) = self.n {
            r = r.json(json!({ "n" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.presence_penalty {
            r = r.json(json!({ "presence_penalty" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.prompt {
            r = r.json(json!({ "prompt" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.stop {
            r = r.json(json!({ "stop" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.stream {
            r = r.json(json!({ "stream" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.suffix {
            r = r.json(json!({ "suffix" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.temperature {
            r = r.json(json!({ "temperature" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.top_p {
            r = r.json(json!({ "top_p" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user {
            r = r.json(json!({ "user" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn best_of(mut self, best_of: i64) -> Self {
        self.best_of = Some(best_of);
        self
    }
    pub fn echo(mut self, echo: bool) -> Self {
        self.echo = Some(echo);
        self
    }
    pub fn frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
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
    pub fn max_tokens(mut self, max_tokens: i64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }
    pub fn n(mut self, n: i64) -> Self {
        self.n = Some(n);
        self
    }
    pub fn presence_penalty(mut self, presence_penalty: f64) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }
    pub fn prompt(mut self, prompt: serde_json::Value) -> Self {
        self.prompt = Some(prompt);
        self
    }
    pub fn stop(mut self, stop: serde_json::Value) -> Self {
        self.stop = Some(stop);
        self
    }
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
    pub fn suffix(mut self, suffix: &str) -> Self {
        self.suffix = Some(suffix.to_owned());
        self
    }
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }
    pub fn top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p);
        self
    }
    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateCompletionRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateCompletionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}