use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListFineTunesRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
}
impl<'a> ListFineTunesRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ListFineTunesResponse> {
        let mut r = self.http_client.client.get("/fine-tunes");
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for ListFineTunesRequest<'a> {
    type Output = httpclient::InMemoryResult<ListFineTunesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}