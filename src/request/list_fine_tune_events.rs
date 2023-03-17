use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListFineTuneEventsRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
    pub fine_tune_id: String,
    pub stream: Option<bool>,
}
impl<'a> ListFineTuneEventsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ListFineTuneEventsResponse> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/fine-tunes/{fine_tune_id}/events", fine_tune_id = self.fine_tune_id
                ),
            );
        if let Some(ref unwrapped) = self.stream {
            r = r.query("stream", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListFineTuneEventsRequest<'a> {
    type Output = httpclient::InMemoryResult<ListFineTuneEventsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}