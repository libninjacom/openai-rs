use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateChatCompletionRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
    pub frequency_penalty: Option<f64>,
    pub logit_bias: Option<serde_json::Value>,
    pub max_tokens: Option<i64>,
    pub messages: Vec<ChatCompletionRequestMessage>,
    pub model: String,
    pub n: Option<i64>,
    pub presence_penalty: Option<f64>,
    pub stop: Option<serde_json::Value>,
    pub stream: Option<bool>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub user: Option<String>,
}
impl<'a> CreateChatCompletionRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CreateChatCompletionResponse> {
        let mut r = self.http_client.client.post("/chat/completions");
        if let Some(ref unwrapped) = self.frequency_penalty {
            r = r.json(json!({ "frequency_penalty" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.logit_bias {
            r = r.json(json!({ "logit_bias" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.max_tokens {
            r = r.json(json!({ "max_tokens" : unwrapped }));
        }
        r = r.json(json!({ "messages" : self.messages }));
        r = r.json(json!({ "model" : self.model }));
        if let Some(ref unwrapped) = self.n {
            r = r.json(json!({ "n" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.presence_penalty {
            r = r.json(json!({ "presence_penalty" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.stop {
            r = r.json(json!({ "stop" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.stream {
            r = r.json(json!({ "stream" : unwrapped }));
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
    pub fn frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }
    pub fn logit_bias(mut self, logit_bias: serde_json::Value) -> Self {
        self.logit_bias = Some(logit_bias);
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
    pub fn stop(mut self, stop: serde_json::Value) -> Self {
        self.stop = Some(stop);
        self
    }
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
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
impl<'a> ::std::future::IntoFuture for CreateChatCompletionRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateChatCompletionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}