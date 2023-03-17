//! [`OpenAiClient`](struct.OpenAiClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;
pub struct OpenAiClient {
    pub client: httpclient::Client,
    authentication: OpenAiAuthentication,
}
impl OpenAiClient {
    pub fn from_env() -> Self {
        Self {
            client: httpclient::Client::new().base_url("https://api.openai.com/v1"),
            authentication: OpenAiAuthentication::from_env(),
        }
    }
}
impl OpenAiClient {
    pub fn new(url: &str, authentication: OpenAiAuthentication) -> Self {
        let client = httpclient::Client::new().base_url(url);
        Self { client, authentication }
    }
    pub fn with_authentication(mut self, authentication: OpenAiAuthentication) -> Self {
        self.authentication = authentication;
        self
    }
    pub(crate) fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            OpenAiAuthentication::Bearer { bearer } => {
                r = r.bearer_auth(bearer);
            }
        }
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
            http_client: &self,
        }
    }
    ///Retrieves a model instance, providing basic information about it such as the owner and availability.
    pub fn retrieve_engine(&self, engine_id: &str) -> request::RetrieveEngineRequest {
        request::RetrieveEngineRequest {
            http_client: &self,
            engine_id: engine_id.to_owned(),
        }
    }
    ///Creates a completion for the provided prompt and parameters
    pub fn create_completion(&self, model: &str) -> request::CreateCompletionRequest {
        request::CreateCompletionRequest {
            http_client: &self,
            best_of: None,
            echo: None,
            frequency_penalty: None,
            logit_bias: None,
            logprobs: None,
            max_tokens: None,
            model: model.to_owned(),
            n: None,
            presence_penalty: None,
            prompt: None,
            stop: None,
            stream: None,
            suffix: None,
            temperature: None,
            top_p: None,
            user: None,
        }
    }
    ///Creates a completion for the chat message
    pub fn create_chat_completion(
        &self,
        messages: Vec<ChatCompletionRequestMessage>,
        model: &str,
    ) -> request::CreateChatCompletionRequest {
        request::CreateChatCompletionRequest {
            http_client: &self,
            frequency_penalty: None,
            logit_bias: None,
            max_tokens: None,
            messages,
            model: model.to_owned(),
            n: None,
            presence_penalty: None,
            stop: None,
            stream: None,
            temperature: None,
            top_p: None,
            user: None,
        }
    }
    ///Creates a new edit for the provided input, instruction, and parameters.
    pub fn create_edit(
        &self,
        instruction: &str,
        model: &str,
    ) -> request::CreateEditRequest {
        request::CreateEditRequest {
            http_client: &self,
            input: None,
            instruction: instruction.to_owned(),
            model: model.to_owned(),
            n: None,
            temperature: None,
            top_p: None,
        }
    }
    ///Creates an image given a prompt.
    pub fn create_image(&self, prompt: &str) -> request::CreateImageRequest {
        request::CreateImageRequest {
            http_client: &self,
            n: None,
            prompt: prompt.to_owned(),
            response_format: None,
            size: None,
            user: None,
        }
    }
    ///Creates an edited or extended image given an original image and a prompt.
    pub fn create_image_edit(&self) -> request::CreateImageEditRequest {
        request::CreateImageEditRequest {
            http_client: &self,
        }
    }
    ///Creates a variation of a given image.
    pub fn create_image_variation(&self) -> request::CreateImageVariationRequest {
        request::CreateImageVariationRequest {
            http_client: &self,
        }
    }
    ///Creates an embedding vector representing the input text.
    pub fn create_embedding(
        &self,
        input: serde_json::Value,
        model: &str,
    ) -> request::CreateEmbeddingRequest {
        request::CreateEmbeddingRequest {
            http_client: &self,
            input,
            model: model.to_owned(),
            user: None,
        }
    }
    ///Transcribes audio into the input language.
    pub fn create_transcription(&self) -> request::CreateTranscriptionRequest {
        request::CreateTranscriptionRequest {
            http_client: &self,
        }
    }
    ///Translates audio into into English.
    pub fn create_translation(&self) -> request::CreateTranslationRequest {
        request::CreateTranslationRequest {
            http_client: &self,
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
            http_client: &self,
            documents: None,
            engine_id: engine_id.to_owned(),
            file: None,
            max_rerank: None,
            query: query.to_owned(),
            return_metadata: None,
            user: None,
        }
    }
    ///Returns a list of files that belong to the user's organization.
    pub fn list_files(&self) -> request::ListFilesRequest {
        request::ListFilesRequest {
            http_client: &self,
        }
    }
    /**Upload a file that contains document(s) to be used across various endpoints/features. Currently, the size of all the files uploaded by one organization can be up to 1 GB. Please contact us if you need to increase the storage limit.
*/
    pub fn create_file(&self) -> request::CreateFileRequest {
        request::CreateFileRequest {
            http_client: &self,
        }
    }
    ///Returns information about a specific file.
    pub fn retrieve_file(&self, file_id: &str) -> request::RetrieveFileRequest {
        request::RetrieveFileRequest {
            http_client: &self,
            file_id: file_id.to_owned(),
        }
    }
    ///Delete a file.
    pub fn delete_file(&self, file_id: &str) -> request::DeleteFileRequest {
        request::DeleteFileRequest {
            http_client: &self,
            file_id: file_id.to_owned(),
        }
    }
    ///Returns the contents of the specified file
    pub fn download_file(&self, file_id: &str) -> request::DownloadFileRequest {
        request::DownloadFileRequest {
            http_client: &self,
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
            http_client: &self,
            documents: None,
            examples: args.examples
                .into_iter()
                .map(|&x| x
                    .into_iter()
                    .map(|&x| x.to_owned())
                    .collect()
                )
                .collect(),
            examples_context: args.examples_context.to_owned(),
            expand: None,
            file: None,
            logit_bias: None,
            logprobs: None,
            max_rerank: None,
            max_tokens: None,
            model: args.model.to_owned(),
            n: None,
            question: args.question.to_owned(),
            return_metadata: None,
            return_prompt: None,
            search_model: None,
            stop: None,
            temperature: None,
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
            http_client: &self,
            examples: None,
            expand: None,
            file: None,
            labels: None,
            logit_bias: None,
            logprobs: None,
            max_examples: None,
            model: model.to_owned(),
            query: query.to_owned(),
            return_metadata: None,
            return_prompt: None,
            search_model: None,
            temperature: None,
            user: None,
        }
    }
    /**List your organization's fine-tuning jobs
*/
    pub fn list_fine_tunes(&self) -> request::ListFineTunesRequest {
        request::ListFineTunesRequest {
            http_client: &self,
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
            http_client: &self,
            batch_size: None,
            classification_betas: None,
            classification_n_classes: None,
            classification_positive_class: None,
            compute_classification_metrics: None,
            learning_rate_multiplier: None,
            model: None,
            n_epochs: None,
            prompt_loss_weight: None,
            suffix: None,
            training_file: training_file.to_owned(),
            validation_file: None,
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
            http_client: &self,
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
            http_client: &self,
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
            http_client: &self,
            fine_tune_id: fine_tune_id.to_owned(),
            stream: None,
        }
    }
    ///Lists the currently available models, and provides basic information about each one such as the owner and availability.
    pub fn list_models(&self) -> request::ListModelsRequest {
        request::ListModelsRequest {
            http_client: &self,
        }
    }
    ///Retrieves a model instance, providing basic information about the model such as the owner and permissioning.
    pub fn retrieve_model(&self, model: &str) -> request::RetrieveModelRequest {
        request::RetrieveModelRequest {
            http_client: &self,
            model: model.to_owned(),
        }
    }
    ///Delete a fine-tuned model. You must have the Owner role in your organization.
    pub fn delete_model(&self, model: &str) -> request::DeleteModelRequest {
        request::DeleteModelRequest {
            http_client: &self,
            model: model.to_owned(),
        }
    }
    ///Classifies if text violates OpenAI's Content Policy
    pub fn create_moderation(
        &self,
        input: serde_json::Value,
    ) -> request::CreateModerationRequest {
        request::CreateModerationRequest {
            http_client: &self,
            input,
            model: None,
        }
    }
}
pub enum OpenAiAuthentication {
    Bearer { bearer: String },
}
impl OpenAiAuthentication {
    pub fn from_env() -> Self {
        Self::Bearer {
            bearer: std::env::var("OPEN_AI_BEARER")
                .expect("Environment variable OPEN_AI_BEARER is not set."),
        }
    }
}