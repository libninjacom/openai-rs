
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAnswerRequired {
    pub examples: Vec<Vec<String>>,
    pub examples_context: String,
    pub model: String,
    pub question: String,
}
impl std::fmt::Display for CreateAnswerRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}