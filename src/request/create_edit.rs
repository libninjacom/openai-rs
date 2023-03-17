use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateEditRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
    pub input: Option<String>,
    pub instruction: String,
    pub model: String,
    pub n: Option<i64>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
}
impl<'a> CreateEditRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateEditResponse> {
        let mut r = self.http_client.client.post("/edits");
        if let Some(ref unwrapped) = self.input {
            r = r.json(json!({ "input" : unwrapped }));
        }
        r = r.json(json!({ "instruction" : self.instruction }));
        r = r.json(json!({ "model" : self.model }));
        if let Some(ref unwrapped) = self.n {
            r = r.json(json!({ "n" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.temperature {
            r = r.json(json!({ "temperature" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.top_p {
            r = r.json(json!({ "top_p" : unwrapped }));
        }
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn input(mut self, input: &str) -> Self {
        self.input = Some(input.to_owned());
        self
    }
    pub fn n(mut self, n: i64) -> Self {
        self.n = Some(n);
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
}
impl<'a> ::std::future::IntoFuture for CreateEditRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateEditResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}