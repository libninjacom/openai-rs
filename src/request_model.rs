use serde_json::json;
use crate::model;
use crate::model::*;
use crate::OpenAiClient;
pub struct ListEnginesRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
}
impl<'a> ListEnginesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ListEnginesResponse> {
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
pub struct RetrieveEngineRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub engine_id: String,
}
impl<'a> RetrieveEngineRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Engine> {
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
pub struct CreateCompletionRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub model: String,
    pub prompt: serde_json::Value,
    pub suffix: String,
    pub max_tokens: i64,
    pub temperature: f64,
    pub top_p: f64,
    pub n: i64,
    pub stream: bool,
    pub logprobs: i64,
    pub echo: bool,
    pub stop: serde_json::Value,
    pub presence_penalty: f64,
    pub frequency_penalty: f64,
    pub best_of: i64,
    pub logit_bias: Option<serde_json::Value>,
    pub user: String,
}
impl<'a> CreateCompletionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CreateCompletionResponse> {
        let mut r = self.client.client.post("/completions");
        r = r.push_json(json!({ "model" : self.model }));
        r = r.push_json(json!({ "prompt" : self.prompt }));
        r = r.push_json(json!({ "suffix" : self.suffix }));
        r = r.push_json(json!({ "max_tokens" : self.max_tokens }));
        r = r.push_json(json!({ "temperature" : self.temperature }));
        r = r.push_json(json!({ "top_p" : self.top_p }));
        r = r.push_json(json!({ "n" : self.n }));
        r = r.push_json(json!({ "stream" : self.stream }));
        r = r.push_json(json!({ "logprobs" : self.logprobs }));
        r = r.push_json(json!({ "echo" : self.echo }));
        r = r.push_json(json!({ "stop" : self.stop }));
        r = r.push_json(json!({ "presence_penalty" : self.presence_penalty }));
        r = r.push_json(json!({ "frequency_penalty" : self.frequency_penalty }));
        r = r.push_json(json!({ "best_of" : self.best_of }));
        if let Some(ref logit_bias) = self.logit_bias {
            r = r.push_json(json!({ "logit_bias" : logit_bias }));
        }
        r = r.push_json(json!({ "user" : self.user }));
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn logit_bias(mut self, logit_bias: serde_json::Value) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }
}
pub struct CreateEditRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub model: String,
    pub input: String,
    pub instruction: String,
    pub n: i64,
    pub temperature: f64,
    pub top_p: f64,
}
impl<'a> CreateEditRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CreateEditResponse> {
        let mut r = self.client.client.post("/edits");
        r = r.push_json(json!({ "model" : self.model }));
        r = r.push_json(json!({ "input" : self.input }));
        r = r.push_json(json!({ "instruction" : self.instruction }));
        r = r.push_json(json!({ "n" : self.n }));
        r = r.push_json(json!({ "temperature" : self.temperature }));
        r = r.push_json(json!({ "top_p" : self.top_p }));
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
pub struct CreateEmbeddingRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub model: String,
    pub input: serde_json::Value,
    pub user: String,
}
impl<'a> CreateEmbeddingRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CreateEmbeddingResponse> {
        let mut r = self.client.client.post("/embeddings");
        r = r.push_json(json!({ "model" : self.model }));
        r = r.push_json(json!({ "input" : self.input }));
        r = r.push_json(json!({ "user" : self.user }));
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
pub struct CreateSearchRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub engine_id: String,
    pub query: String,
    pub documents: Vec<String>,
    pub file: String,
    pub max_rerank: i64,
    pub return_metadata: bool,
    pub user: String,
}
impl<'a> CreateSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CreateSearchResponse> {
        let mut r = self
            .client
            .client
            .post(&format!("/engines/{engine_id}/search", engine_id = self.engine_id));
        r = r.push_json(json!({ "query" : self.query }));
        r = r.push_json(json!({ "documents" : self.documents }));
        r = r.push_json(json!({ "file" : self.file }));
        r = r.push_json(json!({ "max_rerank" : self.max_rerank }));
        r = r.push_json(json!({ "return_metadata" : self.return_metadata }));
        r = r.push_json(json!({ "user" : self.user }));
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
pub struct ListFilesRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
}
impl<'a> ListFilesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ListFilesResponse> {
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
pub struct CreateFileRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
}
impl<'a> CreateFileRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::OpenAiFile> {
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
pub struct RetrieveFileRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub file_id: String,
}
impl<'a> RetrieveFileRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::OpenAiFile> {
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
pub struct DeleteFileRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub file_id: String,
}
impl<'a> DeleteFileRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::DeleteFileResponse> {
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
pub struct DownloadFileRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub file_id: String,
}
impl<'a> DownloadFileRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::String> {
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
pub struct CreateAnswerRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub model: String,
    pub question: String,
    pub examples: Vec<Vec<String>>,
    pub examples_context: String,
    pub documents: Vec<String>,
    pub file: String,
    pub search_model: String,
    pub max_rerank: i64,
    pub temperature: f64,
    pub logprobs: i64,
    pub max_tokens: i64,
    pub stop: serde_json::Value,
    pub n: i64,
    pub logit_bias: Option<serde_json::Value>,
    pub return_metadata: bool,
    pub return_prompt: bool,
    pub expand: Vec<serde_json::Value>,
    pub user: String,
}
impl<'a> CreateAnswerRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CreateAnswerResponse> {
        let mut r = self.client.client.post("/answers");
        r = r.push_json(json!({ "model" : self.model }));
        r = r.push_json(json!({ "question" : self.question }));
        r = r.push_json(json!({ "examples" : self.examples }));
        r = r.push_json(json!({ "examples_context" : self.examples_context }));
        r = r.push_json(json!({ "documents" : self.documents }));
        r = r.push_json(json!({ "file" : self.file }));
        r = r.push_json(json!({ "search_model" : self.search_model }));
        r = r.push_json(json!({ "max_rerank" : self.max_rerank }));
        r = r.push_json(json!({ "temperature" : self.temperature }));
        r = r.push_json(json!({ "logprobs" : self.logprobs }));
        r = r.push_json(json!({ "max_tokens" : self.max_tokens }));
        r = r.push_json(json!({ "stop" : self.stop }));
        r = r.push_json(json!({ "n" : self.n }));
        if let Some(ref logit_bias) = self.logit_bias {
            r = r.push_json(json!({ "logit_bias" : logit_bias }));
        }
        r = r.push_json(json!({ "return_metadata" : self.return_metadata }));
        r = r.push_json(json!({ "return_prompt" : self.return_prompt }));
        r = r.push_json(json!({ "expand" : self.expand }));
        r = r.push_json(json!({ "user" : self.user }));
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn logit_bias(mut self, logit_bias: serde_json::Value) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }
}
pub struct CreateClassificationRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub model: String,
    pub query: String,
    pub examples: Vec<Vec<String>>,
    pub file: String,
    pub labels: Vec<String>,
    pub search_model: String,
    pub temperature: f64,
    pub logprobs: i64,
    pub max_examples: i64,
    pub logit_bias: Option<serde_json::Value>,
    pub return_prompt: bool,
    pub return_metadata: bool,
    pub expand: Vec<serde_json::Value>,
    pub user: String,
}
impl<'a> CreateClassificationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CreateClassificationResponse> {
        let mut r = self.client.client.post("/classifications");
        r = r.push_json(json!({ "model" : self.model }));
        r = r.push_json(json!({ "query" : self.query }));
        r = r.push_json(json!({ "examples" : self.examples }));
        r = r.push_json(json!({ "file" : self.file }));
        r = r.push_json(json!({ "labels" : self.labels }));
        r = r.push_json(json!({ "search_model" : self.search_model }));
        r = r.push_json(json!({ "temperature" : self.temperature }));
        r = r.push_json(json!({ "logprobs" : self.logprobs }));
        r = r.push_json(json!({ "max_examples" : self.max_examples }));
        if let Some(ref logit_bias) = self.logit_bias {
            r = r.push_json(json!({ "logit_bias" : logit_bias }));
        }
        r = r.push_json(json!({ "return_prompt" : self.return_prompt }));
        r = r.push_json(json!({ "return_metadata" : self.return_metadata }));
        r = r.push_json(json!({ "expand" : self.expand }));
        r = r.push_json(json!({ "user" : self.user }));
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn logit_bias(mut self, logit_bias: serde_json::Value) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }
}
pub struct ListFineTunesRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
}
impl<'a> ListFineTunesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ListFineTunesResponse> {
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
pub struct CreateFineTuneRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub training_file: String,
    pub validation_file: String,
    pub model: String,
    pub n_epochs: i64,
    pub batch_size: i64,
    pub learning_rate_multiplier: f64,
    pub prompt_loss_weight: f64,
    pub compute_classification_metrics: bool,
    pub classification_n_classes: i64,
    pub classification_positive_class: String,
    pub classification_betas: Vec<f64>,
    pub suffix: String,
}
impl<'a> CreateFineTuneRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::FineTune> {
        let mut r = self.client.client.post("/fine-tunes");
        r = r.push_json(json!({ "training_file" : self.training_file }));
        r = r.push_json(json!({ "validation_file" : self.validation_file }));
        r = r.push_json(json!({ "model" : self.model }));
        r = r.push_json(json!({ "n_epochs" : self.n_epochs }));
        r = r.push_json(json!({ "batch_size" : self.batch_size }));
        r = r
            .push_json(
                json!({ "learning_rate_multiplier" : self.learning_rate_multiplier }),
            );
        r = r.push_json(json!({ "prompt_loss_weight" : self.prompt_loss_weight }));
        r = r
            .push_json(
                json!(
                    { "compute_classification_metrics" : self
                    .compute_classification_metrics }
                ),
            );
        r = r
            .push_json(
                json!({ "classification_n_classes" : self.classification_n_classes }),
            );
        r = r
            .push_json(
                json!(
                    { "classification_positive_class" : self
                    .classification_positive_class }
                ),
            );
        r = r.push_json(json!({ "classification_betas" : self.classification_betas }));
        r = r.push_json(json!({ "suffix" : self.suffix }));
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
pub struct RetrieveFineTuneRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub fine_tune_id: String,
}
impl<'a> RetrieveFineTuneRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::FineTune> {
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
pub struct CancelFineTuneRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub fine_tune_id: String,
}
impl<'a> CancelFineTuneRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::FineTune> {
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
pub struct ListFineTuneEventsRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub fine_tune_id: String,
    pub stream: Option<bool>,
}
impl<'a> ListFineTuneEventsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ListFineTuneEventsResponse> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/fine-tunes/{fine_tune_id}/events", fine_tune_id = self.fine_tune_id
                ),
            );
        if let Some(ref stream) = self.stream {
            r = r.push_query("stream", &stream.to_string());
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
pub struct ListModelsRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
}
impl<'a> ListModelsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ListModelsResponse> {
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
pub struct RetrieveModelRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub model: String,
}
impl<'a> RetrieveModelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Model> {
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
pub struct DeleteModelRequest<'a> {
    pub(crate) client: &'a OpenAiClient,
    pub model: String,
}
impl<'a> DeleteModelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::DeleteModelResponse> {
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
