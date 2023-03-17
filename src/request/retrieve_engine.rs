use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveEngineRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
    pub engine_id: String,
}
impl<'a> RetrieveEngineRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Engine> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/engines/{engine_id}", engine_id = self.engine_id));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for RetrieveEngineRequest<'a> {
    type Output = httpclient::InMemoryResult<Engine>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}