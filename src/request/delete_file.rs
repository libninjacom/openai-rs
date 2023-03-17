use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteFileRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
    pub file_id: String,
}
impl<'a> DeleteFileRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<DeleteFileResponse> {
        let mut r = self
            .http_client
            .client
            .delete(&format!("/files/{file_id}", file_id = self.file_id));
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for DeleteFileRequest<'a> {
    type Output = httpclient::InMemoryResult<DeleteFileResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}