
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateModerationResponse {
    pub id: String,
    pub model: String,
    pub results: Vec<serde_json::Value>,
}
impl std::fmt::Display for CreateModerationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}