
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAnswerResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_documents: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for CreateAnswerResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}