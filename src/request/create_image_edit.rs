use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateImageEditRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
}
impl<'a> CreateImageEditRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ImagesResponse> {
        let mut r = self.http_client.client.post("/images/edits");
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for CreateImageEditRequest<'a> {
    type Output = httpclient::InMemoryResult<ImagesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}