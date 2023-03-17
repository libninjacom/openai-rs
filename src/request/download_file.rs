use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DownloadFileRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
    pub file_id: String,
}
impl<'a> DownloadFileRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<String> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/files/{file_id}/content", file_id = self.file_id));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for DownloadFileRequest<'a> {
    type Output = httpclient::InMemoryResult<String>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}