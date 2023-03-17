
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmbeddingResponse {
    pub data: Vec<serde_json::Value>,
    pub model: String,
    pub object: String,
    pub usage: serde_json::Value,
}
impl std::fmt::Display for CreateEmbeddingResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}