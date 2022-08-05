use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct ListEnginesResponse {
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[serde(rename = "data")]
    pub data: Option<Vec<Engine>>,
}
impl std::fmt::Display for ListEnginesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListModelsResponse {
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[serde(rename = "data")]
    pub data: Option<Vec<Model>>,
}
impl std::fmt::Display for ListModelsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteModelResponse {
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[serde(rename = "deleted")]
    pub deleted: Option<bool>,
}
impl std::fmt::Display for DeleteModelResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompletionRequest {
    #[serde(rename = "model")]
    ///ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models/overview) for descriptions of them.
    pub model: String,
    #[serde(rename = "prompt")]
    /**The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.

Note that <|endoftext|> is the document separator that the model sees during training, so if a prompt is not specified the model will generate as if from the beginning of a new document.
*/
    pub prompt: Option<serde_json::Value>,
    #[serde(rename = "suffix")]
    ///The suffix that comes after a completion of inserted text.
    pub suffix: Option<String>,
    #[serde(rename = "max_tokens")]
    /**The maximum number of [tokens](/tokenizer) to generate in the completion.

The token count of your prompt plus `max_tokens` cannot exceed the model's context length. Most models have a context length of 2048 tokens (except for the newest models, which support 4096).
*/
    pub max_tokens: Option<i64>,
    #[serde(rename = "temperature")]
    /**What [sampling temperature](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277) to use. Higher values means the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.

We generally recommend altering this or `top_p` but not both.
*/
    pub temperature: Option<f64>,
    #[serde(rename = "top_p")]
    /**An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.

We generally recommend altering this or `temperature` but not both.
*/
    pub top_p: Option<f64>,
    #[serde(rename = "n")]
    /**How many completions to generate for each prompt.

**Note:** Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.
*/
    pub n: Option<i64>,
    #[serde(rename = "stream")]
    /**Whether to stream back partial progress. If set, tokens will be sent as data-only [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format) as they become available, with the stream terminated by a `data: [DONE]` message.
*/
    pub stream: Option<bool>,
    #[serde(rename = "logprobs")]
    /**Include the log probabilities on the `logprobs` most likely tokens, as well the chosen tokens. For example, if `logprobs` is 5, the API will return a list of the 5 most likely tokens. The API will always return the `logprob` of the sampled token, so there may be up to `logprobs+1` elements in the response.

The maximum value for `logprobs` is 5. If you need more than this, please contact support@openai.com and describe your use case.
*/
    pub logprobs: Option<i64>,
    #[serde(rename = "echo")]
    /**Echo back the prompt in addition to the completion
*/
    pub echo: Option<bool>,
    #[serde(rename = "stop")]
    /**Up to 4 sequences where the API will stop generating further tokens. The returned text will not contain the stop sequence.
*/
    pub stop: Option<serde_json::Value>,
    #[serde(rename = "presence_penalty")]
    /**Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.

[See more information about frequency and presence penalties.](/docs/api-reference/parameter-details)
*/
    pub presence_penalty: Option<f64>,
    #[serde(rename = "frequency_penalty")]
    /**Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.

[See more information about frequency and presence penalties.](/docs/api-reference/parameter-details)
*/
    pub frequency_penalty: Option<f64>,
    #[serde(rename = "best_of")]
    /**Generates `best_of` completions server-side and returns the "best" (the one with the highest log probability per token). Results cannot be streamed.

When used with `n`, `best_of` controls the number of candidate completions and `n` specifies how many to return – `best_of` must be greater than `n`.

**Note:** Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.
*/
    pub best_of: Option<i64>,
    #[serde(rename = "logit_bias")]
    /**Modify the likelihood of specified tokens appearing in the completion.

Accepts a json object that maps tokens (specified by their token ID in the GPT tokenizer) to an associated bias value from -100 to 100. You can use this [tokenizer tool](/tokenizer?view=bpe) (which works for both GPT-2 and GPT-3) to convert text to token IDs. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.

As an example, you can pass `{"50256": -100}` to prevent the <|endoftext|> token from being generated.
*/
    pub logit_bias: Option<serde_json::Value>,
    #[serde(rename = "user")]
    /**A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse.
*/
    pub user: Option<String>,
}
impl std::fmt::Display for CreateCompletionRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompletionResponse {
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[serde(rename = "created")]
    pub created: Option<i64>,
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[serde(rename = "choices")]
    pub choices: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for CreateCompletionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEditRequest {
    #[serde(rename = "model")]
    ///ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models/overview) for descriptions of them.
    pub model: String,
    #[serde(rename = "input")]
    ///The input text to use as a starting point for the edit.
    pub input: Option<String>,
    #[serde(rename = "instruction")]
    ///The instruction that tells the model how to edit the prompt.
    pub instruction: String,
    #[serde(rename = "n")]
    ///How many edits to generate for the input and instruction.
    pub n: Option<i64>,
    #[serde(rename = "temperature")]
    /**What [sampling temperature](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277) to use. Higher values means the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.

We generally recommend altering this or `top_p` but not both.
*/
    pub temperature: Option<f64>,
    #[serde(rename = "top_p")]
    /**An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.

We generally recommend altering this or `temperature` but not both.
*/
    pub top_p: Option<f64>,
}
impl std::fmt::Display for CreateEditRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEditResponse {
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[serde(rename = "created")]
    pub created: Option<i64>,
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[serde(rename = "choices")]
    pub choices: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for CreateEditResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSearchRequest {
    #[serde(rename = "query")]
    ///Query to search against the documents.
    pub query: String,
    #[serde(rename = "documents")]
    /**Up to 200 documents to search over, provided as a list of strings.

The maximum document length (in tokens) is 2034 minus the number of tokens in the query.

You should specify either `documents` or a `file`, but not both.
*/
    pub documents: Option<Vec<String>>,
    #[serde(rename = "file")]
    /**The ID of an uploaded file that contains documents to search over.

You should specify either `documents` or a `file`, but not both.
*/
    pub file: Option<String>,
    #[serde(rename = "max_rerank")]
    /**The maximum number of documents to be re-ranked and returned by search.

This flag only takes effect when `file` is set.
*/
    pub max_rerank: Option<i64>,
    #[serde(rename = "return_metadata")]
    /**A special boolean flag for showing metadata. If set to `true`, each document entry in the returned JSON will contain a "metadata" field.

This flag only takes effect when `file` is set.
*/
    pub return_metadata: Option<bool>,
    #[serde(rename = "user")]
    /**A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse.
*/
    pub user: Option<String>,
}
impl std::fmt::Display for CreateSearchRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSearchResponse {
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[serde(rename = "data")]
    pub data: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for CreateSearchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListFilesResponse {
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[serde(rename = "data")]
    pub data: Option<Vec<OpenAiFile>>,
}
impl std::fmt::Display for ListFilesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFileRequest {
    #[serde(rename = "file")]
    /**Name of the [JSON Lines](https://jsonlines.readthedocs.io/en/latest/) file to be uploaded.

If the `purpose` is set to "fine-tune", each line is a JSON record with "prompt" and "completion" fields representing your [training examples](/docs/guides/fine-tuning/prepare-training-data).
*/
    pub file: String,
    #[serde(rename = "purpose")]
    /**The intended purpose of the uploaded documents.

Use "fine-tune" for [Fine-tuning](/docs/api-reference/fine-tunes). This allows us to validate the format of the uploaded file.
*/
    pub purpose: String,
}
impl std::fmt::Display for CreateFileRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteFileResponse {
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[serde(rename = "deleted")]
    pub deleted: Option<bool>,
}
impl std::fmt::Display for DeleteFileResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAnswerRequest {
    #[serde(rename = "model")]
    ///ID of the model to use for completion. You can select one of `ada`, `babbage`, `curie`, or `davinci`.
    pub model: String,
    #[serde(rename = "question")]
    ///Question to get answered.
    pub question: String,
    #[serde(rename = "examples")]
    ///List of (question, answer) pairs that will help steer the model towards the tone and answer format you'd like. We recommend adding 2 to 3 examples.
    pub examples: Vec<Vec<String>>,
    #[serde(rename = "examples_context")]
    ///A text snippet containing the contextual information used to generate the answers for the `examples` you provide.
    pub examples_context: String,
    #[serde(rename = "documents")]
    /**List of documents from which the answer for the input `question` should be derived. If this is an empty list, the question will be answered based on the question-answer examples.

You should specify either `documents` or a `file`, but not both.
*/
    pub documents: Option<Vec<String>>,
    #[serde(rename = "file")]
    /**The ID of an uploaded file that contains documents to search over. See [upload file](/docs/api-reference/files/upload) for how to upload a file of the desired format and purpose.

You should specify either `documents` or a `file`, but not both.
*/
    pub file: Option<String>,
    #[serde(rename = "search_model")]
    ///ID of the model to use for [Search](/docs/api-reference/searches/create). You can select one of `ada`, `babbage`, `curie`, or `davinci`.
    pub search_model: Option<String>,
    #[serde(rename = "max_rerank")]
    ///The maximum number of documents to be ranked by [Search](/docs/api-reference/searches/create) when using `file`. Setting it to a higher value leads to improved accuracy but with increased latency and cost.
    pub max_rerank: Option<i64>,
    #[serde(rename = "temperature")]
    ///What [sampling temperature](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277) to use. Higher values mean the model will take more risks and value 0 (argmax sampling) works better for scenarios with a well-defined answer.
    pub temperature: Option<f64>,
    #[serde(rename = "logprobs")]
    /**Include the log probabilities on the `logprobs` most likely tokens, as well the chosen tokens. For example, if `logprobs` is 5, the API will return a list of the 5 most likely tokens. The API will always return the `logprob` of the sampled token, so there may be up to `logprobs+1` elements in the response.

The maximum value for `logprobs` is 5. If you need more than this, please contact support@openai.com and describe your use case.

When `logprobs` is set, `completion` will be automatically added into `expand` to get the logprobs.
*/
    pub logprobs: Option<i64>,
    #[serde(rename = "max_tokens")]
    ///The maximum number of tokens allowed for the generated answer
    pub max_tokens: Option<i64>,
    #[serde(rename = "stop")]
    /**Up to 4 sequences where the API will stop generating further tokens. The returned text will not contain the stop sequence.
*/
    pub stop: Option<serde_json::Value>,
    #[serde(rename = "n")]
    ///How many answers to generate for each question.
    pub n: Option<i64>,
    #[serde(rename = "logit_bias")]
    /**Modify the likelihood of specified tokens appearing in the completion.

Accepts a json object that maps tokens (specified by their token ID in the GPT tokenizer) to an associated bias value from -100 to 100. You can use this [tokenizer tool](/tokenizer?view=bpe) (which works for both GPT-2 and GPT-3) to convert text to token IDs. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.

As an example, you can pass `{"50256": -100}` to prevent the <|endoftext|> token from being generated.
*/
    pub logit_bias: Option<serde_json::Value>,
    #[serde(rename = "return_metadata")]
    /**A special boolean flag for showing metadata. If set to `true`, each document entry in the returned JSON will contain a "metadata" field.

This flag only takes effect when `file` is set.
*/
    pub return_metadata: Option<bool>,
    #[serde(rename = "return_prompt")]
    ///If set to `true`, the returned JSON will include a "prompt" field containing the final prompt that was used to request a completion. This is mainly useful for debugging purposes.
    pub return_prompt: Option<bool>,
    #[serde(rename = "expand")]
    ///If an object name is in the list, we provide the full information of the object; otherwise, we only provide the object ID. Currently we support `completion` and `file` objects for expansion.
    pub expand: Option<Vec<serde_json::Value>>,
    #[serde(rename = "user")]
    /**A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse.
*/
    pub user: Option<String>,
}
impl std::fmt::Display for CreateAnswerRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAnswerResponse {
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[serde(rename = "search_model")]
    pub search_model: Option<String>,
    #[serde(rename = "completion")]
    pub completion: Option<String>,
    #[serde(rename = "answers")]
    pub answers: Option<Vec<String>>,
    #[serde(rename = "selected_documents")]
    pub selected_documents: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for CreateAnswerResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateClassificationRequest {
    #[serde(rename = "model")]
    ///ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models/overview) for descriptions of them.
    pub model: String,
    #[serde(rename = "query")]
    ///Query to be classified.
    pub query: String,
    #[serde(rename = "examples")]
    /**A list of examples with labels, in the following format:

`[["The movie is so interesting.", "Positive"], ["It is quite boring.", "Negative"], ...]`

All the label strings will be normalized to be capitalized.

You should specify either `examples` or `file`, but not both.
*/
    pub examples: Option<Vec<Vec<String>>>,
    #[serde(rename = "file")]
    /**The ID of the uploaded file that contains training examples. See [upload file](/docs/api-reference/files/upload) for how to upload a file of the desired format and purpose.

You should specify either `examples` or `file`, but not both.
*/
    pub file: Option<String>,
    #[serde(rename = "labels")]
    ///The set of categories being classified. If not specified, candidate labels will be automatically collected from the examples you provide. All the label strings will be normalized to be capitalized.
    pub labels: Option<Vec<String>>,
    #[serde(rename = "search_model")]
    ///ID of the model to use for [Search](/docs/api-reference/searches/create). You can select one of `ada`, `babbage`, `curie`, or `davinci`.
    pub search_model: Option<String>,
    #[serde(rename = "temperature")]
    ///What sampling `temperature` to use. Higher values mean the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    pub temperature: Option<f64>,
    #[serde(rename = "logprobs")]
    /**Include the log probabilities on the `logprobs` most likely tokens, as well the chosen tokens. For example, if `logprobs` is 5, the API will return a list of the 5 most likely tokens. The API will always return the `logprob` of the sampled token, so there may be up to `logprobs+1` elements in the response.

The maximum value for `logprobs` is 5. If you need more than this, please contact support@openai.com and describe your use case.

When `logprobs` is set, `completion` will be automatically added into `expand` to get the logprobs.
*/
    pub logprobs: Option<i64>,
    #[serde(rename = "max_examples")]
    ///The maximum number of examples to be ranked by [Search](/docs/api-reference/searches/create) when using `file`. Setting it to a higher value leads to improved accuracy but with increased latency and cost.
    pub max_examples: Option<i64>,
    #[serde(rename = "logit_bias")]
    /**Modify the likelihood of specified tokens appearing in the completion.

Accepts a json object that maps tokens (specified by their token ID in the GPT tokenizer) to an associated bias value from -100 to 100. You can use this [tokenizer tool](/tokenizer?view=bpe) (which works for both GPT-2 and GPT-3) to convert text to token IDs. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.

As an example, you can pass `{"50256": -100}` to prevent the <|endoftext|> token from being generated.
*/
    pub logit_bias: Option<serde_json::Value>,
    #[serde(rename = "return_prompt")]
    ///If set to `true`, the returned JSON will include a "prompt" field containing the final prompt that was used to request a completion. This is mainly useful for debugging purposes.
    pub return_prompt: Option<bool>,
    #[serde(rename = "return_metadata")]
    /**A special boolean flag for showing metadata. If set to `true`, each document entry in the returned JSON will contain a "metadata" field.

This flag only takes effect when `file` is set.
*/
    pub return_metadata: Option<bool>,
    #[serde(rename = "expand")]
    ///If an object name is in the list, we provide the full information of the object; otherwise, we only provide the object ID. Currently we support `completion` and `file` objects for expansion.
    pub expand: Option<Vec<serde_json::Value>>,
    #[serde(rename = "user")]
    /**A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse.
*/
    pub user: Option<String>,
}
impl std::fmt::Display for CreateClassificationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateClassificationResponse {
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[serde(rename = "search_model")]
    pub search_model: Option<String>,
    #[serde(rename = "completion")]
    pub completion: Option<String>,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "selected_examples")]
    pub selected_examples: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for CreateClassificationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFineTuneRequest {
    #[serde(rename = "training_file")]
    /**The ID of an uploaded file that contains training data.

See [upload file](/docs/api-reference/files/upload) for how to upload a file.

Your dataset must be formatted as a JSONL file, where each training
example is a JSON object with the keys "prompt" and "completion".
Additionally, you must upload your file with the purpose `fine-tune`.

See the [fine-tuning guide](/docs/guides/fine-tuning/creating-training-data) for more details.
*/
    pub training_file: String,
    #[serde(rename = "validation_file")]
    /**The ID of an uploaded file that contains validation data.

If you provide this file, the data is used to generate validation
metrics periodically during fine-tuning. These metrics can be viewed in
the [fine-tuning results file](/docs/guides/fine-tuning/analyzing-your-fine-tuned-model).
Your train and validation data should be mutually exclusive.

Your dataset must be formatted as a JSONL file, where each validation
example is a JSON object with the keys "prompt" and "completion".
Additionally, you must upload your file with the purpose `fine-tune`.

See the [fine-tuning guide](/docs/guides/fine-tuning/creating-training-data) for more details.
*/
    pub validation_file: Option<String>,
    #[serde(rename = "model")]
    /**The name of the base model to fine-tune. You can select one of "ada",
"babbage", "curie", or "davinci". To learn more about these models, see the
[Models](https://beta.openai.com/docs/models) documentation.
*/
    pub model: Option<String>,
    #[serde(rename = "n_epochs")]
    /**The number of epochs to train the model for. An epoch refers to one
full cycle through the training dataset.
*/
    pub n_epochs: Option<i64>,
    #[serde(rename = "batch_size")]
    /**The batch size to use for training. The batch size is the number of
training examples used to train a single forward and backward pass.

By default, the batch size will be dynamically configured to be
~0.2% of the number of examples in the training set, capped at 256 -
in general, we've found that larger batch sizes tend to work better
for larger datasets.
*/
    pub batch_size: Option<i64>,
    #[serde(rename = "learning_rate_multiplier")]
    /**The learning rate multiplier to use for training.
The fine-tuning learning rate is the original learning rate used for
pretraining multiplied by this value.

By default, the learning rate multiplier is the 0.05, 0.1, or 0.2
depending on final `batch_size` (larger learning rates tend to
perform better with larger batch sizes). We recommend experimenting
with values in the range 0.02 to 0.2 to see what produces the best
results.
*/
    pub learning_rate_multiplier: Option<f64>,
    #[serde(rename = "prompt_loss_weight")]
    /**The weight to use for loss on the prompt tokens. This controls how
much the model tries to learn to generate the prompt (as compared
to the completion which always has a weight of 1.0), and can add
a stabilizing effect to training when completions are short.

If prompts are extremely long (relative to completions), it may make
sense to reduce this weight so as to avoid over-prioritizing
learning the prompt.
*/
    pub prompt_loss_weight: Option<f64>,
    #[serde(rename = "compute_classification_metrics")]
    /**If set, we calculate classification-specific metrics such as accuracy
and F-1 score using the validation set at the end of every epoch.
These metrics can be viewed in the [results file](/docs/guides/fine-tuning/analyzing-your-fine-tuned-model).

In order to compute classification metrics, you must provide a
`validation_file`. Additionally, you must
specify `classification_n_classes` for multiclass classification or
`classification_positive_class` for binary classification.
*/
    pub compute_classification_metrics: Option<bool>,
    #[serde(rename = "classification_n_classes")]
    /**The number of classes in a classification task.

This parameter is required for multiclass classification.
*/
    pub classification_n_classes: Option<i64>,
    #[serde(rename = "classification_positive_class")]
    /**The positive class in binary classification.

This parameter is needed to generate precision, recall, and F1
metrics when doing binary classification.
*/
    pub classification_positive_class: Option<String>,
    #[serde(rename = "classification_betas")]
    /**If this is provided, we calculate F-beta scores at the specified
beta values. The F-beta score is a generalization of F-1 score.
This is only used for binary classification.

With a beta of 1 (i.e. the F-1 score), precision and recall are
given the same weight. A larger beta score puts more weight on
recall and less on precision. A smaller beta score puts more weight
on precision and less on recall.
*/
    pub classification_betas: Option<Vec<f64>>,
    #[serde(rename = "suffix")]
    /**A string of up to 40 characters that will be added to your fine-tuned model name.

For example, a `suffix` of "custom-model-name" would produce a model name like `ada:ft-your-org:custom-model-name-2022-02-15-04-21-04`.
*/
    pub suffix: Option<String>,
}
impl std::fmt::Display for CreateFineTuneRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListFineTunesResponse {
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[serde(rename = "data")]
    pub data: Option<Vec<FineTune>>,
}
impl std::fmt::Display for ListFineTunesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListFineTuneEventsResponse {
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[serde(rename = "data")]
    pub data: Option<Vec<FineTuneEvent>>,
}
impl std::fmt::Display for ListFineTuneEventsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmbeddingRequest {
    #[serde(rename = "model")]
    ///ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models/overview) for descriptions of them.
    pub model: String,
    #[serde(rename = "input")]
    /**Input text to get embeddings for, encoded as a string or array of tokens. To get embeddings for multiple inputs in a single request, pass an array of strings or array of token arrays. Each input must not exceed 2048 tokens in length.

Unless your are embedding code, we suggest replacing newlines (`\n`) in your input with a single space, as we have observed inferior results when newlines are present.
*/
    pub input: serde_json::Value,
    #[serde(rename = "user")]
    /**A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse.
*/
    pub user: Option<String>,
}
impl std::fmt::Display for CreateEmbeddingRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmbeddingResponse {
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[serde(rename = "data")]
    pub data: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for CreateEmbeddingResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Engine(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct Model(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAiFile(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct FineTune(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct FineTuneEvent(pub serde_json::Value);
