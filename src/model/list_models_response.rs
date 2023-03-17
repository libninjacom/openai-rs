
use serde::{Serialize, Deserialize};
use super::Model;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListModelsResponse {
    pub data: Vec<Model>,
    pub object: String,
}
impl std::fmt::Display for ListModelsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}