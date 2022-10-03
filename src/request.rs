use serde_json::json;
use crate::model::*;
use crate::OpenAiClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListEnginesRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
}
impl<'a> ListEnginesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ListEnginesResponse> {
        let mut r = self.client.client.get("/engines");
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RetrieveEngineRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub engine_id: String,
}
impl<'a> RetrieveEngineRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Engine> {
        let mut r = self
            .client
            .client
            .get(&format!("/engines/{engine_id}", engine_id = self.engine_id));
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateCompletionRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub model: String,
    pub prompt: Option<serde_json::Value>,
    pub suffix: Option<String>,
    pub max_tokens: Option<i64>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub n: Option<i64>,
    pub stream: Option<bool>,
    pub logprobs: Option<i64>,
    pub echo: Option<bool>,
    pub stop: Option<serde_json::Value>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub best_of: Option<i64>,
    pub logit_bias: Option<serde_json::Value>,
    pub user: Option<String>,
}
impl<'a> CreateCompletionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreateCompletionResponse> {
        let mut r = self.client.client.post("/completions");
        r = r.push_json(json!({ "model" : self.model }));
        if let Some(ref unwrapped) = self.prompt {
            r = r.push_json(json!({ "prompt" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.suffix {
            r = r.push_json(json!({ "suffix" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.max_tokens {
            r = r.push_json(json!({ "max_tokens" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.temperature {
            r = r.push_json(json!({ "temperature" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.top_p {
            r = r.push_json(json!({ "top_p" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.n {
            r = r.push_json(json!({ "n" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.stream {
            r = r.push_json(json!({ "stream" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.logprobs {
            r = r.push_json(json!({ "logprobs" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.echo {
            r = r.push_json(json!({ "echo" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.stop {
            r = r.push_json(json!({ "stop" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.presence_penalty {
            r = r.push_json(json!({ "presence_penalty" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.frequency_penalty {
            r = r.push_json(json!({ "frequency_penalty" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.best_of {
            r = r.push_json(json!({ "best_of" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.logit_bias {
            r = r.push_json(json!({ "logit_bias" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user {
            r = r.push_json(json!({ "user" : unwrapped }));
        }
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn prompt(mut self, prompt: serde_json::Value) -> Self {
        self.prompt = Some(prompt);
        self
    }
    pub fn suffix(mut self, suffix: &str) -> Self {
        self.suffix = Some(suffix.to_owned());
        self
    }
    pub fn max_tokens(mut self, max_tokens: i64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }
    pub fn top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p);
        self
    }
    pub fn n(mut self, n: i64) -> Self {
        self.n = Some(n);
        self
    }
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
    pub fn logprobs(mut self, logprobs: i64) -> Self {
        self.logprobs = Some(logprobs);
        self
    }
    pub fn echo(mut self, echo: bool) -> Self {
        self.echo = Some(echo);
        self
    }
    pub fn stop(mut self, stop: serde_json::Value) -> Self {
        self.stop = Some(stop);
        self
    }
    pub fn presence_penalty(mut self, presence_penalty: f64) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }
    pub fn frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }
    pub fn best_of(mut self, best_of: i64) -> Self {
        self.best_of = Some(best_of);
        self
    }
    pub fn logit_bias(mut self, logit_bias: serde_json::Value) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }
    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateEditRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub model: String,
    pub input: Option<String>,
    pub instruction: String,
    pub n: Option<i64>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
}
impl<'a> CreateEditRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreateEditResponse> {
        let mut r = self.client.client.post("/edits");
        r = r.push_json(json!({ "model" : self.model }));
        if let Some(ref unwrapped) = self.input {
            r = r.push_json(json!({ "input" : unwrapped }));
        }
        r = r.push_json(json!({ "instruction" : self.instruction }));
        if let Some(ref unwrapped) = self.n {
            r = r.push_json(json!({ "n" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.temperature {
            r = r.push_json(json!({ "temperature" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.top_p {
            r = r.push_json(json!({ "top_p" : unwrapped }));
        }
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn input(mut self, input: &str) -> Self {
        self.input = Some(input.to_owned());
        self
    }
    pub fn n(mut self, n: i64) -> Self {
        self.n = Some(n);
        self
    }
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }
    pub fn top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateEmbeddingRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub model: String,
    pub input: serde_json::Value,
    pub user: Option<String>,
}
impl<'a> CreateEmbeddingRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreateEmbeddingResponse> {
        let mut r = self.client.client.post("/embeddings");
        r = r.push_json(json!({ "model" : self.model }));
        r = r.push_json(json!({ "input" : self.input }));
        if let Some(ref unwrapped) = self.user {
            r = r.push_json(json!({ "user" : unwrapped }));
        }
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateSearchRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub engine_id: String,
    pub query: String,
    pub documents: Option<Vec<String>>,
    pub file: Option<String>,
    pub max_rerank: Option<i64>,
    pub return_metadata: Option<bool>,
    pub user: Option<String>,
}
impl<'a> CreateSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreateSearchResponse> {
        let mut r = self
            .client
            .client
            .post(&format!("/engines/{engine_id}/search", engine_id = self.engine_id));
        r = r.push_json(json!({ "query" : self.query }));
        if let Some(ref unwrapped) = self.documents {
            r = r.push_json(json!({ "documents" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.file {
            r = r.push_json(json!({ "file" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.max_rerank {
            r = r.push_json(json!({ "max_rerank" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.return_metadata {
            r = r.push_json(json!({ "return_metadata" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user {
            r = r.push_json(json!({ "user" : unwrapped }));
        }
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListFilesRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
}
impl<'a> ListFilesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ListFilesResponse> {
        let mut r = self.client.client.get("/files");
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateFileRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
}
impl<'a> CreateFileRequest<'a> {
    pub async fn send(self) -> anyhow::Result<OpenAiFile> {
        let mut r = self.client.client.post("/files");
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RetrieveFileRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub file_id: String,
}
impl<'a> RetrieveFileRequest<'a> {
    pub async fn send(self) -> anyhow::Result<OpenAiFile> {
        let mut r = self
            .client
            .client
            .get(&format!("/files/{file_id}", file_id = self.file_id));
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteFileRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub file_id: String,
}
impl<'a> DeleteFileRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeleteFileResponse> {
        let mut r = self
            .client
            .client
            .delete(&format!("/files/{file_id}", file_id = self.file_id));
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DownloadFileRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub file_id: String,
}
impl<'a> DownloadFileRequest<'a> {
    pub async fn send(self) -> anyhow::Result<String> {
        let mut r = self
            .client
            .client
            .get(&format!("/files/{file_id}/content", file_id = self.file_id));
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateAnswerRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub model: String,
    pub question: String,
    pub examples: Vec<Vec<String>>,
    pub examples_context: String,
    pub documents: Option<Vec<String>>,
    pub file: Option<String>,
    pub search_model: Option<String>,
    pub max_rerank: Option<i64>,
    pub temperature: Option<f64>,
    pub logprobs: Option<i64>,
    pub max_tokens: Option<i64>,
    pub stop: Option<serde_json::Value>,
    pub n: Option<i64>,
    pub logit_bias: Option<serde_json::Value>,
    pub return_metadata: Option<bool>,
    pub return_prompt: Option<bool>,
    pub expand: Option<Vec<serde_json::Value>>,
    pub user: Option<String>,
}
impl<'a> CreateAnswerRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreateAnswerResponse> {
        let mut r = self.client.client.post("/answers");
        r = r.push_json(json!({ "model" : self.model }));
        r = r.push_json(json!({ "question" : self.question }));
        r = r.push_json(json!({ "examples" : self.examples }));
        r = r.push_json(json!({ "examples_context" : self.examples_context }));
        if let Some(ref unwrapped) = self.documents {
            r = r.push_json(json!({ "documents" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.file {
            r = r.push_json(json!({ "file" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.search_model {
            r = r.push_json(json!({ "search_model" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.max_rerank {
            r = r.push_json(json!({ "max_rerank" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.temperature {
            r = r.push_json(json!({ "temperature" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.logprobs {
            r = r.push_json(json!({ "logprobs" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.max_tokens {
            r = r.push_json(json!({ "max_tokens" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.stop {
            r = r.push_json(json!({ "stop" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.n {
            r = r.push_json(json!({ "n" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.logit_bias {
            r = r.push_json(json!({ "logit_bias" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.return_metadata {
            r = r.push_json(json!({ "return_metadata" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.return_prompt {
            r = r.push_json(json!({ "return_prompt" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.expand {
            r = r.push_json(json!({ "expand" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user {
            r = r.push_json(json!({ "user" : unwrapped }));
        }
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
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
    pub fn search_model(mut self, search_model: &str) -> Self {
        self.search_model = Some(search_model.to_owned());
        self
    }
    pub fn max_rerank(mut self, max_rerank: i64) -> Self {
        self.max_rerank = Some(max_rerank);
        self
    }
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }
    pub fn logprobs(mut self, logprobs: i64) -> Self {
        self.logprobs = Some(logprobs);
        self
    }
    pub fn max_tokens(mut self, max_tokens: i64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }
    pub fn stop(mut self, stop: serde_json::Value) -> Self {
        self.stop = Some(stop);
        self
    }
    pub fn n(mut self, n: i64) -> Self {
        self.n = Some(n);
        self
    }
    pub fn logit_bias(mut self, logit_bias: serde_json::Value) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }
    pub fn return_metadata(mut self, return_metadata: bool) -> Self {
        self.return_metadata = Some(return_metadata);
        self
    }
    pub fn return_prompt(mut self, return_prompt: bool) -> Self {
        self.return_prompt = Some(return_prompt);
        self
    }
    pub fn expand(mut self, expand: Vec<serde_json::Value>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_owned());
        self
    }
}
pub struct CreateAnswerRequired<'a> {
    pub model: &'a str,
    pub question: &'a str,
    pub examples: &'a [&'a [&'a str]],
    pub examples_context: &'a str,
}
impl<'a> CreateAnswerRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateClassificationRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub model: String,
    pub query: String,
    pub examples: Option<Vec<Vec<String>>>,
    pub file: Option<String>,
    pub labels: Option<Vec<String>>,
    pub search_model: Option<String>,
    pub temperature: Option<f64>,
    pub logprobs: Option<i64>,
    pub max_examples: Option<i64>,
    pub logit_bias: Option<serde_json::Value>,
    pub return_prompt: Option<bool>,
    pub return_metadata: Option<bool>,
    pub expand: Option<Vec<serde_json::Value>>,
    pub user: Option<String>,
}
impl<'a> CreateClassificationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreateClassificationResponse> {
        let mut r = self.client.client.post("/classifications");
        r = r.push_json(json!({ "model" : self.model }));
        r = r.push_json(json!({ "query" : self.query }));
        if let Some(ref unwrapped) = self.examples {
            r = r.push_json(json!({ "examples" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.file {
            r = r.push_json(json!({ "file" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.labels {
            r = r.push_json(json!({ "labels" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.search_model {
            r = r.push_json(json!({ "search_model" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.temperature {
            r = r.push_json(json!({ "temperature" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.logprobs {
            r = r.push_json(json!({ "logprobs" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.max_examples {
            r = r.push_json(json!({ "max_examples" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.logit_bias {
            r = r.push_json(json!({ "logit_bias" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.return_prompt {
            r = r.push_json(json!({ "return_prompt" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.return_metadata {
            r = r.push_json(json!({ "return_metadata" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.expand {
            r = r.push_json(json!({ "expand" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user {
            r = r.push_json(json!({ "user" : unwrapped }));
        }
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn examples(mut self, examples: &[&[&str]]) -> Self {
        self.examples = Some(examples.to_owned());
        self
    }
    pub fn file(mut self, file: &str) -> Self {
        self.file = Some(file.to_owned());
        self
    }
    pub fn labels(mut self, labels: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.labels = Some(labels.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn search_model(mut self, search_model: &str) -> Self {
        self.search_model = Some(search_model.to_owned());
        self
    }
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }
    pub fn logprobs(mut self, logprobs: i64) -> Self {
        self.logprobs = Some(logprobs);
        self
    }
    pub fn max_examples(mut self, max_examples: i64) -> Self {
        self.max_examples = Some(max_examples);
        self
    }
    pub fn logit_bias(mut self, logit_bias: serde_json::Value) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }
    pub fn return_prompt(mut self, return_prompt: bool) -> Self {
        self.return_prompt = Some(return_prompt);
        self
    }
    pub fn return_metadata(mut self, return_metadata: bool) -> Self {
        self.return_metadata = Some(return_metadata);
        self
    }
    pub fn expand(mut self, expand: Vec<serde_json::Value>) -> Self {
        self.expand = Some(expand);
        self
    }
    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListFineTunesRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
}
impl<'a> ListFineTunesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ListFineTunesResponse> {
        let mut r = self.client.client.get("/fine-tunes");
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateFineTuneRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub training_file: String,
    pub validation_file: Option<String>,
    pub model: Option<String>,
    pub n_epochs: Option<i64>,
    pub batch_size: Option<i64>,
    pub learning_rate_multiplier: Option<f64>,
    pub prompt_loss_weight: Option<f64>,
    pub compute_classification_metrics: Option<bool>,
    pub classification_n_classes: Option<i64>,
    pub classification_positive_class: Option<String>,
    pub classification_betas: Option<Vec<f64>>,
    pub suffix: Option<String>,
}
impl<'a> CreateFineTuneRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FineTune> {
        let mut r = self.client.client.post("/fine-tunes");
        r = r.push_json(json!({ "training_file" : self.training_file }));
        if let Some(ref unwrapped) = self.validation_file {
            r = r.push_json(json!({ "validation_file" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.model {
            r = r.push_json(json!({ "model" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.n_epochs {
            r = r.push_json(json!({ "n_epochs" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.batch_size {
            r = r.push_json(json!({ "batch_size" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.learning_rate_multiplier {
            r = r.push_json(json!({ "learning_rate_multiplier" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.prompt_loss_weight {
            r = r.push_json(json!({ "prompt_loss_weight" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.compute_classification_metrics {
            r = r.push_json(json!({ "compute_classification_metrics" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.classification_n_classes {
            r = r.push_json(json!({ "classification_n_classes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.classification_positive_class {
            r = r.push_json(json!({ "classification_positive_class" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.classification_betas {
            r = r.push_json(json!({ "classification_betas" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.suffix {
            r = r.push_json(json!({ "suffix" : unwrapped }));
        }
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn validation_file(mut self, validation_file: &str) -> Self {
        self.validation_file = Some(validation_file.to_owned());
        self
    }
    pub fn model(mut self, model: &str) -> Self {
        self.model = Some(model.to_owned());
        self
    }
    pub fn n_epochs(mut self, n_epochs: i64) -> Self {
        self.n_epochs = Some(n_epochs);
        self
    }
    pub fn batch_size(mut self, batch_size: i64) -> Self {
        self.batch_size = Some(batch_size);
        self
    }
    pub fn learning_rate_multiplier(mut self, learning_rate_multiplier: f64) -> Self {
        self.learning_rate_multiplier = Some(learning_rate_multiplier);
        self
    }
    pub fn prompt_loss_weight(mut self, prompt_loss_weight: f64) -> Self {
        self.prompt_loss_weight = Some(prompt_loss_weight);
        self
    }
    pub fn compute_classification_metrics(
        mut self,
        compute_classification_metrics: bool,
    ) -> Self {
        self.compute_classification_metrics = Some(compute_classification_metrics);
        self
    }
    pub fn classification_n_classes(mut self, classification_n_classes: i64) -> Self {
        self.classification_n_classes = Some(classification_n_classes);
        self
    }
    pub fn classification_positive_class(
        mut self,
        classification_positive_class: &str,
    ) -> Self {
        self
            .classification_positive_class = Some(
            classification_positive_class.to_owned(),
        );
        self
    }
    pub fn classification_betas(mut self, classification_betas: Vec<f64>) -> Self {
        self.classification_betas = Some(classification_betas);
        self
    }
    pub fn suffix(mut self, suffix: &str) -> Self {
        self.suffix = Some(suffix.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RetrieveFineTuneRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub fine_tune_id: String,
}
impl<'a> RetrieveFineTuneRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FineTune> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/fine-tunes/{fine_tune_id}", fine_tune_id = self.fine_tune_id),
            );
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CancelFineTuneRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub fine_tune_id: String,
}
impl<'a> CancelFineTuneRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FineTune> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/fine-tunes/{fine_tune_id}/cancel", fine_tune_id = self.fine_tune_id
                ),
            );
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListFineTuneEventsRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub fine_tune_id: String,
    pub stream: Option<bool>,
}
impl<'a> ListFineTuneEventsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ListFineTuneEventsResponse> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/fine-tunes/{fine_tune_id}/events", fine_tune_id = self.fine_tune_id
                ),
            );
        if let Some(ref unwrapped) = self.stream {
            r = r.push_query("stream", &unwrapped.to_string());
        }
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListModelsRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
}
impl<'a> ListModelsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ListModelsResponse> {
        let mut r = self.client.client.get("/models");
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RetrieveModelRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub model: String,
}
impl<'a> RetrieveModelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Model> {
        let mut r = self
            .client
            .client
            .get(&format!("/models/{model}", model = self.model));
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteModelRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub model: String,
}
impl<'a> DeleteModelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeleteModelResponse> {
        let mut r = self
            .client
            .client
            .delete(&format!("/models/{model}", model = self.model));
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
