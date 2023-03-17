use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateEmbeddingRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
    pub input: serde_json::Value,
    pub model: String,
    pub user: Option<String>,
}
impl<'a> CreateEmbeddingRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateEmbeddingResponse> {
        let mut r = self.http_client.client.post("/embeddings");
        r = r.json(json!({ "input" : self.input }));
        r = r.json(json!({ "model" : self.model }));
        if let Some(ref unwrapped) = self.user {
            r = r.json(json!({ "user" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateEmbeddingRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateEmbeddingResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}