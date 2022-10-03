//! [`OpenAiClient`](struct.OpenAiClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;

pub struct OpenAiClient {
    pub(crate) client: httpclient::Client,
    authentication: OpenAiAuthentication,
}
impl OpenAiClient {
    pub fn from_env() -> Self {
        let url = "https://api.openai.com/v1".to_string();
        Self {
            client: httpclient::Client::new(Some(url)),
            authentication: open_ai_authentication::from_env(),
        }
    }
}
impl OpenAiClient {
    pub fn new(url: &str, authentication: OpenAiAuthentication) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        Self { client, authentication }
    }
    pub fn with_authentication(mut self, authentication: OpenAiAuthentication) -> Self {
        self.authentication = authentication;
        self
    }
    pub fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {}
        r
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    ///Lists the currently available (non-finetuned) models, and provides basic information about each one such as the owner and availability.
    pub fn list_engines(&self) -> request::ListEnginesRequest {
        request::ListEnginesRequest {
            client: &self,
        }
    }
    ///Retrieves a model instance, providing basic information about it such as the owner and availability.
    pub fn retrieve_engine(&self, engine_id: &str) -> request::RetrieveEngineRequest {
        request::RetrieveEngineRequest {
            client: &self,
            engine_id: engine_id.to_owned(),
        }
    }
    ///Creates a completion for the provided prompt and parameters
    pub fn create_completion(&self, model: &str) -> request::CreateCompletionRequest {
        request::CreateCompletionRequest {
            client: &self,
            model: model.to_owned(),
            prompt: None,
            suffix: None,
            max_tokens: None,
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            logprobs: None,
            echo: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            best_of: None,
            logit_bias: None,
            user: None,
        }
    }
    ///Creates a new edit for the provided input, instruction, and parameters
    pub fn create_edit(
        &self,
        model: &str,
        instruction: &str,
    ) -> request::CreateEditRequest {
        request::CreateEditRequest {
            client: &self,
            model: model.to_owned(),
            input: None,
            instruction: instruction.to_owned(),
            n: None,
            temperature: None,
            top_p: None,
        }
    }
    ///Creates an embedding vector representing the input text.
    pub fn create_embedding(
        &self,
        model: &str,
        input: serde_json::Value,
    ) -> request::CreateEmbeddingRequest {
        request::CreateEmbeddingRequest {
            client: &self,
            model: model.to_owned(),
            input,
            user: None,
        }
    }
    /**The search endpoint computes similarity scores between provided query and documents. Documents can be passed directly to the API if there are no more than 200 of them.

To go beyond the 200 document limit, documents can be processed offline and then used for efficient retrieval at query time. When `file` is set, the search endpoint searches over all the documents in the given file and returns up to the `max_rerank` number of documents. These documents will be returned along with their search scores.

The similarity score is a positive score that usually ranges from 0 to 300 (but can sometimes go higher), where a score above 200 usually means the document is semantically similar to the query.
*/
    pub fn create_search(
        &self,
        engine_id: &str,
        query: &str,
    ) -> request::CreateSearchRequest {
        request::CreateSearchRequest {
            client: &self,
            engine_id: engine_id.to_owned(),
            query: query.to_owned(),
            documents: None,
            file: None,
            max_rerank: None,
            return_metadata: None,
            user: None,
        }
    }
    ///Returns a list of files that belong to the user's organization.
    pub fn list_files(&self) -> request::ListFilesRequest {
        request::ListFilesRequest {
            client: &self,
        }
    }
    /**Upload a file that contains document(s) to be used across various endpoints/features. Currently, the size of all the files uploaded by one organization can be up to 1 GB. Please contact us if you need to increase the storage limit.
*/
    pub fn create_file(&self) -> request::CreateFileRequest {
        request::CreateFileRequest {
            client: &self,
        }
    }
    ///Returns information about a specific file.
    pub fn retrieve_file(&self, file_id: &str) -> request::RetrieveFileRequest {
        request::RetrieveFileRequest {
            client: &self,
            file_id: file_id.to_owned(),
        }
    }
    ///Delete a file.
    pub fn delete_file(&self, file_id: &str) -> request::DeleteFileRequest {
        request::DeleteFileRequest {
            client: &self,
            file_id: file_id.to_owned(),
        }
    }
    ///Returns the contents of the specified file
    pub fn download_file(&self, file_id: &str) -> request::DownloadFileRequest {
        request::DownloadFileRequest {
            client: &self,
            file_id: file_id.to_owned(),
        }
    }
    /**Answers the specified question using the provided documents and examples.

The endpoint first [searches](/docs/api-reference/searches) over provided documents or files to find relevant context. The relevant context is combined with the provided examples and question to create the prompt for [completion](/docs/api-reference/completions).
*/
    pub fn create_answer(
        &self,
        args: request::CreateAnswerRequired,
    ) -> request::CreateAnswerRequest {
        request::CreateAnswerRequest {
            client: &self,
            model: args.model.to_owned(),
            question: args.question.to_owned(),
            examples: args.examples.iter().map(|&x| x.to_owned()).collect(),
            examples_context: args.examples_context.to_owned(),
            documents: None,
            file: None,
            search_model: None,
            max_rerank: None,
            temperature: None,
            logprobs: None,
            max_tokens: None,
            stop: None,
            n: None,
            logit_bias: None,
            return_metadata: None,
            return_prompt: None,
            expand: None,
            user: None,
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
        model: &str,
        query: &str,
    ) -> request::CreateClassificationRequest {
        request::CreateClassificationRequest {
            client: &self,
            model: model.to_owned(),
            query: query.to_owned(),
            examples: None,
            file: None,
            labels: None,
            search_model: None,
            temperature: None,
            logprobs: None,
            max_examples: None,
            logit_bias: None,
            return_prompt: None,
            return_metadata: None,
            expand: None,
            user: None,
        }
    }
    /**List your organization's fine-tuning jobs
*/
    pub fn list_fine_tunes(&self) -> request::ListFineTunesRequest {
        request::ListFineTunesRequest {
            client: &self,
        }
    }
    /**Creates a job that fine-tunes a specified model from a given dataset.

Response includes details of the enqueued job including job status and the name of the fine-tuned models once complete.

[Learn more about Fine-tuning](/docs/guides/fine-tuning)
*/
    pub fn create_fine_tune(
        &self,
        training_file: &str,
    ) -> request::CreateFineTuneRequest {
        request::CreateFineTuneRequest {
            client: &self,
            training_file: training_file.to_owned(),
            validation_file: None,
            model: None,
            n_epochs: None,
            batch_size: None,
            learning_rate_multiplier: None,
            prompt_loss_weight: None,
            compute_classification_metrics: None,
            classification_n_classes: None,
            classification_positive_class: None,
            classification_betas: None,
            suffix: None,
        }
    }
    /**Gets info about the fine-tune job.

[Learn more about Fine-tuning](/docs/guides/fine-tuning)
*/
    pub fn retrieve_fine_tune(
        &self,
        fine_tune_id: &str,
    ) -> request::RetrieveFineTuneRequest {
        request::RetrieveFineTuneRequest {
            client: &self,
            fine_tune_id: fine_tune_id.to_owned(),
        }
    }
    /**Immediately cancel a fine-tune job.
*/
    pub fn cancel_fine_tune(
        &self,
        fine_tune_id: &str,
    ) -> request::CancelFineTuneRequest {
        request::CancelFineTuneRequest {
            client: &self,
            fine_tune_id: fine_tune_id.to_owned(),
        }
    }
    /**Get fine-grained status updates for a fine-tune job.
*/
    pub fn list_fine_tune_events(
        &self,
        fine_tune_id: &str,
    ) -> request::ListFineTuneEventsRequest {
        request::ListFineTuneEventsRequest {
            client: &self,
            fine_tune_id: fine_tune_id.to_owned(),
            stream: None,
        }
    }
    ///Lists the currently available models, and provides basic information about each one such as the owner and availability.
    pub fn list_models(&self) -> request::ListModelsRequest {
        request::ListModelsRequest {
            client: &self,
        }
    }
    ///Retrieves a model instance, providing basic information about the model such as the owner and permissioning.
    pub fn retrieve_model(&self, model: &str) -> request::RetrieveModelRequest {
        request::RetrieveModelRequest {
            client: &self,
            model: model.to_owned(),
        }
    }
    ///Delete a fine-tuned model. You must have the Owner role in your organization.
    pub fn delete_model(&self, model: &str) -> request::DeleteModelRequest {
        request::DeleteModelRequest {
            client: &self,
            model: model.to_owned(),
        }
    }
}
