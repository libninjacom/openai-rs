use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateTranscriptionRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
}
impl<'a> CreateTranscriptionRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CreateTranscriptionResponse> {
        let mut r = self.http_client.client.post("/audio/transcriptions");
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for CreateTranscriptionRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateTranscriptionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}