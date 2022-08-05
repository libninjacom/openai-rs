//! [`OpenAiClient`](struct.OpenAiClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
pub mod model;
pub mod request_model;
use crate::model::*;

pub struct OpenAiClient {
    pub(crate) client: httpclient::Client,
}
impl OpenAiClient {}
impl OpenAiClient {
    pub fn new(url: &str) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        OpenAiClient { client }
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    ///Lists the currently available (non-finetuned) models, and provides basic information about each one such as the owner and availability.
    pub fn list_engines(&self) -> request_model::ListEnginesRequest {
        request_model::ListEnginesRequest {
            client: &self,
        }
    }
    ///Retrieves a model instance, providing basic information about it such as the owner and availability.
    pub fn retrieve_engine(
        &self,
        engine_id: String,
    ) -> request_model::RetrieveEngineRequest {
        request_model::RetrieveEngineRequest {
            client: &self,
            engine_id,
        }
    }
    ///Creates a completion for the provided prompt and parameters
    pub fn create_completion(
        &self,
        model: String,
        prompt: serde_json::Value,
        suffix: String,
        max_tokens: i64,
        temperature: f64,
        top_p: f64,
        n: i64,
        stream: bool,
        logprobs: i64,
        echo: bool,
        stop: serde_json::Value,
        presence_penalty: f64,
        frequency_penalty: f64,
        best_of: i64,
        user: String,
    ) -> request_model::CreateCompletionRequest {
        request_model::CreateCompletionRequest {
            client: &self,
            model,
            prompt,
            suffix,
            max_tokens,
            temperature,
            top_p,
            n,
            stream,
            logprobs,
            echo,
            stop,
            presence_penalty,
            frequency_penalty,
            best_of,
            logit_bias: None,
            user,
        }
    }
    ///Creates a new edit for the provided input, instruction, and parameters
    pub fn create_edit(
        &self,
        model: String,
        input: String,
        instruction: String,
        n: i64,
        temperature: f64,
        top_p: f64,
    ) -> request_model::CreateEditRequest {
        request_model::CreateEditRequest {
            client: &self,
            model,
            input,
            instruction,
            n,
            temperature,
            top_p,
        }
    }
    ///Creates an embedding vector representing the input text.
    pub fn create_embedding(
        &self,
        model: String,
        input: serde_json::Value,
        user: String,
    ) -> request_model::CreateEmbeddingRequest {
        request_model::CreateEmbeddingRequest {
            client: &self,
            model,
            input,
            user,
        }
    }
    /**The search endpoint computes similarity scores between provided query and documents. Documents can be passed directly to the API if there are no more than 200 of them.

To go beyond the 200 document limit, documents can be processed offline and then used for efficient retrieval at query time. When `file` is set, the search endpoint searches over all the documents in the given file and returns up to the `max_rerank` number of documents. These documents will be returned along with their search scores.

The similarity score is a positive score that usually ranges from 0 to 300 (but can sometimes go higher), where a score above 200 usually means the document is semantically similar to the query.
*/
    pub fn create_search(
        &self,
        engine_id: String,
        query: String,
        documents: Vec<String>,
        file: String,
        max_rerank: i64,
        return_metadata: bool,
        user: String,
    ) -> request_model::CreateSearchRequest {
        request_model::CreateSearchRequest {
            client: &self,
            engine_id,
            query,
            documents,
            file,
            max_rerank,
            return_metadata,
            user,
        }
    }
    ///Returns a list of files that belong to the user's organization.
    pub fn list_files(&self) -> request_model::ListFilesRequest {
        request_model::ListFilesRequest {
            client: &self,
        }
    }
    /**Upload a file that contains document(s) to be used across various endpoints/features. Currently, the size of all the files uploaded by one organization can be up to 1 GB. Please contact us if you need to increase the storage limit.
*/
    pub fn create_file(&self) -> request_model::CreateFileRequest {
        request_model::CreateFileRequest {
            client: &self,
        }
    }
    ///Returns information about a specific file.
    pub fn retrieve_file(&self, file_id: String) -> request_model::RetrieveFileRequest {
        request_model::RetrieveFileRequest {
            client: &self,
            file_id,
        }
    }
    ///Delete a file.
    pub fn delete_file(&self, file_id: String) -> request_model::DeleteFileRequest {
        request_model::DeleteFileRequest {
            client: &self,
            file_id,
        }
    }
    ///Returns the contents of the specified file
    pub fn download_file(&self, file_id: String) -> request_model::DownloadFileRequest {
        request_model::DownloadFileRequest {
            client: &self,
            file_id,
        }
    }
    /**Answers the specified question using the provided documents and examples.

The endpoint first [searches](/docs/api-reference/searches) over provided documents or files to find relevant context. The relevant context is combined with the provided examples and question to create the prompt for [completion](/docs/api-reference/completions).
*/
    pub fn create_answer(
        &self,
        model: String,
        question: String,
        examples: Vec<Vec<String>>,
        examples_context: String,
        documents: Vec<String>,
        file: String,
        search_model: String,
        max_rerank: i64,
        temperature: f64,
        logprobs: i64,
        max_tokens: i64,
        stop: serde_json::Value,
        n: i64,
        return_metadata: bool,
        return_prompt: bool,
        expand: Vec<serde_json::Value>,
        user: String,
    ) -> request_model::CreateAnswerRequest {
        request_model::CreateAnswerRequest {
            client: &self,
            model,
            question,
            examples,
            examples_context,
            documents,
            file,
            search_model,
            max_rerank,
            temperature,
            logprobs,
            max_tokens,
            stop,
            n,
            logit_bias: None,
            return_metadata,
            return_prompt,
            expand,
            user,
        }
    }
    /**Classifies the specified `query` using provided examples.

The endpoint first [searches](/docs/api-reference/searches) over the labeled examples
to select the ones most relevant for the particular query. Then, the relevant examples
are combined with the query to construct a prompt to produce the final label via the
[completions](/docs/api-reference/completions) endpoint.

Labeled examples can be provided via an uploaded `file`, or explicitly listed in the
request using the `examples` parameter for quick tests and small scale use cases.
*/
    pub fn create_classification(
        &self,
        model: String,
        query: String,
        examples: Vec<Vec<String>>,
        file: String,
        labels: Vec<String>,
        search_model: String,
        temperature: f64,
        logprobs: i64,
        max_examples: i64,
        return_prompt: bool,
        return_metadata: bool,
        expand: Vec<serde_json::Value>,
        user: String,
    ) -> request_model::CreateClassificationRequest {
        request_model::CreateClassificationRequest {
            client: &self,
            model,
            query,
            examples,
            file,
            labels,
            search_model,
            temperature,
            logprobs,
            max_examples,
            logit_bias: None,
            return_prompt,
            return_metadata,
            expand,
            user,
        }
    }
    /**List your organization's fine-tuning jobs
*/
    pub fn list_fine_tunes(&self) -> request_model::ListFineTunesRequest {
        request_model::ListFineTunesRequest {
            client: &self,
        }
    }
    /**Creates a job that fine-tunes a specified model from a given dataset.

Response includes details of the enqueued job including job status and the name of the fine-tuned models once complete.

[Learn more about Fine-tuning](/docs/guides/fine-tuning)
*/
    pub fn create_fine_tune(
        &self,
        training_file: String,
        validation_file: String,
        model: String,
        n_epochs: i64,
        batch_size: i64,
        learning_rate_multiplier: f64,
        prompt_loss_weight: f64,
        compute_classification_metrics: bool,
        classification_n_classes: i64,
        classification_positive_class: String,
        classification_betas: Vec<f64>,
        suffix: String,
    ) -> request_model::CreateFineTuneRequest {
        request_model::CreateFineTuneRequest {
            client: &self,
            training_file,
            validation_file,
            model,
            n_epochs,
            batch_size,
            learning_rate_multiplier,
            prompt_loss_weight,
            compute_classification_metrics,
            classification_n_classes,
            classification_positive_class,
            classification_betas,
            suffix,
        }
    }
    /**Gets info about the fine-tune job.

[Learn more about Fine-tuning](/docs/guides/fine-tuning)
*/
    pub fn retrieve_fine_tune(
        &self,
        fine_tune_id: String,
    ) -> request_model::RetrieveFineTuneRequest {
        request_model::RetrieveFineTuneRequest {
            client: &self,
            fine_tune_id,
        }
    }
    /**Immediately cancel a fine-tune job.
*/
    pub fn cancel_fine_tune(
        &self,
        fine_tune_id: String,
    ) -> request_model::CancelFineTuneRequest {
        request_model::CancelFineTuneRequest {
            client: &self,
            fine_tune_id,
        }
    }
    /**Get fine-grained status updates for a fine-tune job.
*/
    pub fn list_fine_tune_events(
        &self,
        fine_tune_id: String,
    ) -> request_model::ListFineTuneEventsRequest {
        request_model::ListFineTuneEventsRequest {
            client: &self,
            fine_tune_id,
            stream: None,
        }
    }
    ///Lists the currently available models, and provides basic information about each one such as the owner and availability.
    pub fn list_models(&self) -> request_model::ListModelsRequest {
        request_model::ListModelsRequest {
            client: &self,
        }
    }
    ///Retrieves a model instance, providing basic information about the model such as the owner and permissioning.
    pub fn retrieve_model(&self, model: String) -> request_model::RetrieveModelRequest {
        request_model::RetrieveModelRequest {
            client: &self,
            model,
        }
    }
    ///Delete a fine-tuned model. You must have the Owner role in your organization.
    pub fn delete_model(&self, model: String) -> request_model::DeleteModelRequest {
        request_model::DeleteModelRequest {
            client: &self,
            model,
        }
    }
}
