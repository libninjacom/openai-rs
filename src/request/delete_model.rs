use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteModelRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
    pub model: String,
}
impl<'a> DeleteModelRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<DeleteModelResponse> {
        let mut r = self
            .http_client
            .client
            .delete(&format!("/models/{model}", model = self.model));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for DeleteModelRequest<'a> {
    type Output = httpclient::InMemoryResult<DeleteModelResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}