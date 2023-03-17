use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateTranslationRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
}
impl<'a> CreateTranslationRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateTranslationResponse> {
        let mut r = self.http_client.client.post("/audio/translations");
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for CreateTranslationRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateTranslationResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}