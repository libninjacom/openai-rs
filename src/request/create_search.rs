use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateSearchRequest<'a> {
    pub(crate) http_client: &'a OpenAiClient,
    pub documents: Option<Vec<String>>,
    pub engine_id: String,
    pub file: Option<String>,
    pub max_rerank: Option<i64>,
    pub query: String,
    pub return_metadata: Option<bool>,
    pub user: Option<String>,
}
impl<'a> CreateSearchRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreateSearchResponse> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/engines/{engine_id}/search", engine_id = self.engine_id));
        if let Some(ref unwrapped) = self.documents {
            r = r.json(json!({ "documents" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.file {
            r = r.json(json!({ "file" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.max_rerank {
            r = r.json(json!({ "max_rerank" : unwrapped }));
        }
        r = r.json(json!({ "query" : self.query }));
        if let Some(ref unwrapped) = self.return_metadata {
            r = r.json(json!({ "return_metadata" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user {
            r = r.json(json!({ "user" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn documents(
        mut self,
        documents: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .documents = Some(
            documents.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn file(mut self, file: &str) -> Self {
        self.file = Some(file.to_owned());
        self
    }
    pub fn max_rerank(mut self, max_rerank: i64) -> Self {
        self.max_rerank = Some(max_rerank);
        self
    }
    pub fn return_metadata(mut self, return_metadata: bool) -> Self {
        self.return_metadata = Some(return_metadata);
        self
    }
    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateSearchRequest<'a> {
    type Output = httpclient::InMemoryResult<CreateSearchResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}