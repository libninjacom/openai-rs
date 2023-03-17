use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateImageRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
    pub n: Option<i64>,
    pub prompt: String,
    pub response_format: Option<String>,
    pub size: Option<String>,
    pub user: Option<String>,
}
impl<'a> CreateImageRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ImagesResponse> {
        let mut r = self.http_client.client.post("/images/generations");
        if let Some(ref unwrapped) = self.n {
            r = r.json(json!({ "n" : unwrapped }));
        }
        r = r.json(json!({ "prompt" : self.prompt }));
        if let Some(ref unwrapped) = self.response_format {
            r = r.json(json!({ "response_format" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.size {
            r = r.json(json!({ "size" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user {
            r = r.json(json!({ "user" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn n(mut self, n: i64) -> Self {
        self.n = Some(n);
        self
    }
    pub fn response_format(mut self, response_format: &str) -> Self {
        self.response_format = Some(response_format.to_owned());
        self
    }
    pub fn size(mut self, size: &str) -> Self {
        self.size = Some(size.to_owned());
        self
    }
    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateImageRequest<'a> {
    type Output = httpclient::InMemoryResult<ImagesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}