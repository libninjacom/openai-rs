use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateModerationRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
    pub input: serde_json::Value,
    pub model: Option<String>,
}
impl<'a> CreateModerationRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateModerationResponse> {
        let mut r = self.http_client.client.post("/moderations");
        r = r.json(json!({ "input" : self.input }));
        if let Some(ref unwrapped) = self.model {
            r = r.json(json!({ "model" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn model(mut self, model: &str) -> Self {
        self.model = Some(model.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateModerationRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateModerationResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}