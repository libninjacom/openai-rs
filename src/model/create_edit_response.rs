
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEditResponse {
    pub choices: Vec<serde_json::Value>,
    pub created: i64,
    pub object: String,
    pub usage: serde_json::Value,
}
impl std::fmt::Display for CreateEditResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}