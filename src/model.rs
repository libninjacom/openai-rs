use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListEnginesResponse {
    pub object: Option<String>,
    pub data: Option<Vec<Engine>>,
}
impl std::fmt::Display for ListEnginesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListModelsResponse {
    pub object: Option<String>,
    pub data: Option<Vec<Model>>,
}
impl std::fmt::Display for ListModelsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeleteModelResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub deleted: Option<bool>,
}
impl std::fmt::Display for DeleteModelResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateCompletionRequest {
    ///ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models/overview) for descriptions of them.
    pub model: String,
    /**The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.

Note that <|endoftext|> is the document separator that the model sees during training, so if a prompt is not specified the model will generate as if from the beginning of a new document.
*/
    pub prompt: Option<serde_json::Value>,
    ///The suffix that comes after a completion of inserted text.
    pub suffix: Option<String>,
    /**The maximum number of [tokens](/tokenizer) to generate in the completion.

The token count of your prompt plus `max_tokens` cannot exceed the model's context length. Most models have a context length of 2048 tokens (except for the newest models, which support 4096).
*/
    pub max_tokens: Option<i64>,
    /**What [sampling temperature](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277) to use. Higher values means the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.

We generally recommend altering this or `top_p` but not both.
*/
    pub temperature: Option<f64>,
    /**An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.

We generally recommend altering this or `temperature` but not both.
*/
    pub top_p: Option<f64>,
    /**How many completions to generate for each prompt.

**Note:** Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.
*/
    pub n: Option<i64>,
    /**Whether to stream back partial progress. If set, tokens will be sent as data-only [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format) as they become available, with the stream terminated by a `data: [DONE]` message.
*/
    pub stream: Option<bool>,
    /**Include the log probabilities on the `logprobs` most likely tokens, as well the chosen tokens. For example, if `logprobs` is 5, the API will return a list of the 5 most likely tokens. The API will always return the `logprob` of the sampled token, so there may be up to `logprobs+1` elements in the response.

The maximum value for `logprobs` is 5. If you need more than this, please contact support@openai.com and describe your use case.
*/
    pub logprobs: Option<i64>,
    /**Echo back the prompt in addition to the completion
*/
    pub echo: Option<bool>,
    /**Up to 4 sequences where the API will stop generating further tokens. The returned text will not contain the stop sequence.
*/
    pub stop: Option<serde_json::Value>,
    /**Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.

[See more information about frequency and presence penalties.](/docs/api-reference/parameter-details)
*/
    pub presence_penalty: Option<f64>,
    /**Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.

[See more information about frequency and presence penalties.](/docs/api-reference/parameter-details)
*/
    pub frequency_penalty: Option<f64>,
    /**Generates `best_of` completions server-side and returns the "best" (the one with the highest log probability per token). Results cannot be streamed.

When used with `n`, `best_of` controls the number of candidate completions and `n` specifies how many to return – `best_of` must be greater than `n`.

**Note:** Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.
*/
    pub best_of: Option<i64>,
    /**Modify the likelihood of specified tokens appearing in the completion.

Accepts a json object that maps tokens (specified by their token ID in the GPT tokenizer) to an associated bias value from -100 to 100. You can use this [tokenizer tool](/tokenizer?view=bpe) (which works for both GPT-2 and GPT-3) to convert text to token IDs. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.

As an example, you can pass `{"50256": -100}` to prevent the <|endoftext|> token from being generated.
*/
    pub logit_bias: Option<serde_json::Value>,
    /**A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse.
*/
    pub user: Option<String>,
}
impl std::fmt::Display for CreateCompletionRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateCompletionResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub model: Option<String>,
    pub choices: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for CreateCompletionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateEditRequest {
    ///ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models/overview) for descriptions of them.
    pub model: String,
    ///The input text to use as a starting point for the edit.
    pub input: Option<String>,
    ///The instruction that tells the model how to edit the prompt.
    pub instruction: String,
    ///How many edits to generate for the input and instruction.
    pub n: Option<i64>,
    /**What [sampling temperature](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277) to use. Higher values means the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.

We generally recommend altering this or `top_p` but not both.
*/
    pub temperature: Option<f64>,
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateEditResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub model: Option<String>,
    pub choices: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for CreateEditResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateSearchRequest {
    ///Query to search against the documents.
    pub query: String,
    /**Up to 200 documents to search over, provided as a list of strings.

The maximum document length (in tokens) is 2034 minus the number of tokens in the query.

You should specify either `documents` or a `file`, but not both.
*/
    pub documents: Option<Vec<String>>,
    /**The ID of an uploaded file that contains documents to search over.

You should specify either `documents` or a `file`, but not both.
*/
    pub file: Option<String>,
    /**The maximum number of documents to be re-ranked and returned by search.

This flag only takes effect when `file` is set.
*/
    pub max_rerank: Option<i64>,
    /**A special boolean flag for showing metadata. If set to `true`, each document entry in the returned JSON will contain a "metadata" field.

This flag only takes effect when `file` is set.
*/
    pub return_metadata: Option<bool>,
    /**A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse.
*/
    pub user: Option<String>,
}
impl std::fmt::Display for CreateSearchRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateSearchResponse {
    pub object: Option<String>,
    pub model: Option<String>,
    pub data: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for CreateSearchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListFilesResponse {
    pub object: Option<String>,
    pub data: Option<Vec<OpenAiFile>>,
}
impl std::fmt::Display for ListFilesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateFileRequest {
    /**Name of the [JSON Lines](https://jsonlines.readthedocs.io/en/latest/) file to be uploaded.

If the `purpose` is set to "fine-tune", each line is a JSON record with "prompt" and "completion" fields representing your [training examples](/docs/guides/fine-tuning/prepare-training-data).
*/
    pub file: String,
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeleteFileResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub deleted: Option<bool>,
}
impl std::fmt::Display for DeleteFileResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateAnswerRequest {
    ///ID of the model to use for completion. You can select one of `ada`, `babbage`, `curie`, or `davinci`.
    pub model: String,
    ///Question to get answered.
    pub question: String,
    ///List of (question, answer) pairs that will help steer the model towards the tone and answer format you'd like. We recommend adding 2 to 3 examples.
    pub examples: Vec<Vec<String>>,
    ///A text snippet containing the contextual information used to generate the answers for the `examples` you provide.
    pub examples_context: String,
    /**List of documents from which the answer for the input `question` should be derived. If this is an empty list, the question will be answered based on the question-answer examples.

You should specify either `documents` or a `file`, but not both.
*/
    pub documents: Option<Vec<String>>,
    /**The ID of an uploaded file that contains documents to search over. See [upload file](/docs/api-reference/files/upload) for how to upload a file of the desired format and purpose.

You should specify either `documents` or a `file`, but not both.
*/
    pub file: Option<String>,
    ///ID of the model to use for [Search](/docs/api-reference/searches/create). You can select one of `ada`, `babbage`, `curie`, or `davinci`.
    pub search_model: Option<String>,
    ///The maximum number of documents to be ranked by [Search](/docs/api-reference/searches/create) when using `file`. Setting it to a higher value leads to improved accuracy but with increased latency and cost.
    pub max_rerank: Option<i64>,
    ///What [sampling temperature](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277) to use. Higher values mean the model will take more risks and value 0 (argmax sampling) works better for scenarios with a well-defined answer.
    pub temperature: Option<f64>,
    /**Include the log probabilities on the `logprobs` most likely tokens, as well the chosen tokens. For example, if `logprobs` is 5, the API will return a list of the 5 most likely tokens. The API will always return the `logprob` of the sampled token, so there may be up to `logprobs+1` elements in the response.

The maximum value for `logprobs` is 5. If you need more than this, please contact support@openai.com and describe your use case.

When `logprobs` is set, `completion` will be automatically added into `expand` to get the logprobs.
*/
    pub logprobs: Option<i64>,
    ///The maximum number of tokens allowed for the generated answer
    pub max_tokens: Option<i64>,
    /**Up to 4 sequences where the API will stop generating further tokens. The returned text will not contain the stop sequence.
*/
    pub stop: Option<serde_json::Value>,
    ///How many answers to generate for each question.
    pub n: Option<i64>,
    /**Modify the likelihood of specified tokens appearing in the completion.

Accepts a json object that maps tokens (specified by their token ID in the GPT tokenizer) to an associated bias value from -100 to 100. You can use this [tokenizer tool](/tokenizer?view=bpe) (which works for both GPT-2 and GPT-3) to convert text to token IDs. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.

As an example, you can pass `{"50256": -100}` to prevent the <|endoftext|> token from being generated.
*/
    pub logit_bias: Option<serde_json::Value>,
    /**A special boolean flag for showing metadata. If set to `true`, each document entry in the returned JSON will contain a "metadata" field.

This flag only takes effect when `file` is set.
*/
    pub return_metadata: Option<bool>,
    ///If set to `true`, the returned JSON will include a "prompt" field containing the final prompt that was used to request a completion. This is mainly useful for debugging purposes.
    pub return_prompt: Option<bool>,
    ///If an object name is in the list, we provide the full information of the object; otherwise, we only provide the object ID. Currently we support `completion` and `file` objects for expansion.
    pub expand: Option<Vec<serde_json::Value>>,
    /**A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse.
*/
    pub user: Option<String>,
}
impl std::fmt::Display for CreateAnswerRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateAnswerResponse {
    pub object: Option<String>,
    pub model: Option<String>,
    pub search_model: Option<String>,
    pub completion: Option<String>,
    pub answers: Option<Vec<String>>,
    pub selected_documents: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for CreateAnswerResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateClassificationRequest {
    ///ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models/overview) for descriptions of them.
    pub model: String,
    ///Query to be classified.
    pub query: String,
    /**A list of examples with labels, in the following format:

`[["The movie is so interesting.", "Positive"], ["It is quite boring.", "Negative"], ...]`

All the label strings will be normalized to be capitalized.

You should specify either `examples` or `file`, but not both.
*/
    pub examples: Option<Vec<Vec<String>>>,
    /**The ID of the uploaded file that contains training examples. See [upload file](/docs/api-reference/files/upload) for how to upload a file of the desired format and purpose.

You should specify either `examples` or `file`, but not both.
*/
    pub file: Option<String>,
    ///The set of categories being classified. If not specified, candidate labels will be automatically collected from the examples you provide. All the label strings will be normalized to be capitalized.
    pub labels: Option<Vec<String>>,
    ///ID of the model to use for [Search](/docs/api-reference/searches/create). You can select one of `ada`, `babbage`, `curie`, or `davinci`.
    pub search_model: Option<String>,
    ///What sampling `temperature` to use. Higher values mean the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    pub temperature: Option<f64>,
    /**Include the log probabilities on the `logprobs` most likely tokens, as well the chosen tokens. For example, if `logprobs` is 5, the API will return a list of the 5 most likely tokens. The API will always return the `logprob` of the sampled token, so there may be up to `logprobs+1` elements in the response.

The maximum value for `logprobs` is 5. If you need more than this, please contact support@openai.com and describe your use case.

When `logprobs` is set, `completion` will be automatically added into `expand` to get the logprobs.
*/
    pub logprobs: Option<i64>,
    ///The maximum number of examples to be ranked by [Search](/docs/api-reference/searches/create) when using `file`. Setting it to a higher value leads to improved accuracy but with increased latency and cost.
    pub max_examples: Option<i64>,
    /**Modify the likelihood of specified tokens appearing in the completion.

Accepts a json object that maps tokens (specified by their token ID in the GPT tokenizer) to an associated bias value from -100 to 100. You can use this [tokenizer tool](/tokenizer?view=bpe) (which works for both GPT-2 and GPT-3) to convert text to token IDs. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.

As an example, you can pass `{"50256": -100}` to prevent the <|endoftext|> token from being generated.
*/
    pub logit_bias: Option<serde_json::Value>,
    ///If set to `true`, the returned JSON will include a "prompt" field containing the final prompt that was used to request a completion. This is mainly useful for debugging purposes.
    pub return_prompt: Option<bool>,
    /**A special boolean flag for showing metadata. If set to `true`, each document entry in the returned JSON will contain a "metadata" field.

This flag only takes effect when `file` is set.
*/
    pub return_metadata: Option<bool>,
    ///If an object name is in the list, we provide the full information of the object; otherwise, we only provide the object ID. Currently we support `completion` and `file` objects for expansion.
    pub expand: Option<Vec<serde_json::Value>>,
    /**A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse.
*/
    pub user: Option<String>,
}
impl std::fmt::Display for CreateClassificationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateClassificationResponse {
    pub object: Option<String>,
    pub model: Option<String>,
    pub search_model: Option<String>,
    pub completion: Option<String>,
    pub label: Option<String>,
    pub selected_examples: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for CreateClassificationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateFineTuneRequest {
    /**The ID of an uploaded file that contains training data.

See [upload file](/docs/api-reference/files/upload) for how to upload a file.

Your dataset must be formatted as a JSONL file, where each training
example is a JSON object with the keys "prompt" and "completion".
Additionally, you must upload your file with the purpose `fine-tune`.

See the [fine-tuning guide](/docs/guides/fine-tuning/creating-training-data) for more details.
*/
    pub training_file: String,
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
    /**The name of the base model to fine-tune. You can select one of "ada",
"babbage", "curie", or "davinci". To learn more about these models, see the
[Models](https://beta.openai.com/docs/models) documentation.
*/
    pub model: Option<String>,
    /**The number of epochs to train the model for. An epoch refers to one
full cycle through the training dataset.
*/
    pub n_epochs: Option<i64>,
    /**The batch size to use for training. The batch size is the number of
training examples used to train a single forward and backward pass.

By default, the batch size will be dynamically configured to be
~0.2% of the number of examples in the training set, capped at 256 -
in general, we've found that larger batch sizes tend to work better
for larger datasets.
*/
    pub batch_size: Option<i64>,
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
    /**The weight to use for loss on the prompt tokens. This controls how
much the model tries to learn to generate the prompt (as compared
to the completion which always has a weight of 1.0), and can add
a stabilizing effect to training when completions are short.

If prompts are extremely long (relative to completions), it may make
sense to reduce this weight so as to avoid over-prioritizing
learning the prompt.
*/
    pub prompt_loss_weight: Option<f64>,
    /**If set, we calculate classification-specific metrics such as accuracy
and F-1 score using the validation set at the end of every epoch.
These metrics can be viewed in the [results file](/docs/guides/fine-tuning/analyzing-your-fine-tuned-model).

In order to compute classification metrics, you must provide a
`validation_file`. Additionally, you must
specify `classification_n_classes` for multiclass classification or
`classification_positive_class` for binary classification.
*/
    pub compute_classification_metrics: Option<bool>,
    /**The number of classes in a classification task.

This parameter is required for multiclass classification.
*/
    pub classification_n_classes: Option<i64>,
    /**The positive class in binary classification.

This parameter is needed to generate precision, recall, and F1
metrics when doing binary classification.
*/
    pub classification_positive_class: Option<String>,
    /**If this is provided, we calculate F-beta scores at the specified
beta values. The F-beta score is a generalization of F-1 score.
This is only used for binary classification.

With a beta of 1 (i.e. the F-1 score), precision and recall are
given the same weight. A larger beta score puts more weight on
recall and less on precision. A smaller beta score puts more weight
on precision and less on recall.
*/
    pub classification_betas: Option<Vec<f64>>,
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListFineTunesResponse {
    pub object: Option<String>,
    pub data: Option<Vec<FineTune>>,
}
impl std::fmt::Display for ListFineTunesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListFineTuneEventsResponse {
    pub object: Option<String>,
    pub data: Option<Vec<FineTuneEvent>>,
}
impl std::fmt::Display for ListFineTuneEventsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmbeddingRequest {
    ///ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models/overview) for descriptions of them.
    pub model: String,
    /**Input text to get embeddings for, encoded as a string or array of tokens. To get embeddings for multiple inputs in a single request, pass an array of strings or array of token arrays. Each input must not exceed 2048 tokens in length.

Unless your are embedding code, we suggest replacing newlines (`\n`) in your input with a single space, as we have observed inferior results when newlines are present.
*/
    pub input: serde_json::Value,
    /**A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse.
*/
    pub user: Option<String>,
}
impl std::fmt::Display for CreateEmbeddingRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateEmbeddingResponse {
    pub object: Option<String>,
    pub model: Option<String>,
    pub data: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for CreateEmbeddingResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Engine {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub ready: Option<bool>,
}
impl std::fmt::Display for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Model {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub owned_by: Option<String>,
}
impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OpenAiFile {
    pub id: Option<String>,
    pub object: Option<String>,
    pub bytes: Option<i64>,
    pub created_at: Option<i64>,
    pub filename: Option<String>,
    pub purpose: Option<String>,
    pub status: Option<String>,
    pub status_details: Option<serde_json::Value>,
}
impl std::fmt::Display for OpenAiFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FineTune {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
    pub model: Option<String>,
    pub fine_tuned_model: Option<String>,
    pub organization_id: Option<String>,
    pub status: Option<String>,
    pub hyperparams: Option<serde_json::Value>,
    pub training_files: Option<Vec<OpenAiFile>>,
    pub validation_files: Option<Vec<OpenAiFile>>,
    pub result_files: Option<Vec<OpenAiFile>>,
    pub events: Option<Vec<FineTuneEvent>>,
}
impl std::fmt::Display for FineTune {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FineTuneEvent {
    pub object: Option<String>,
    pub created_at: Option<i64>,
    pub level: Option<String>,
    pub message: Option<String>,
}
impl std::fmt::Display for FineTuneEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
