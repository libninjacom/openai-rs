use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CancelFineTuneRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
    pub fine_tune_id: String,
}
impl<'a> CancelFineTuneRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<FineTune> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/fine-tunes/{fine_tune_id}/cancel", fine_tune_id = self.fine_tune_id
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for CancelFineTuneRequest<'a> {
    type Output = httpclient::InMemoryResult<FineTune>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}