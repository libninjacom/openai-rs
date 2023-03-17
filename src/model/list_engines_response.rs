
use serde::{Serialize, Deserialize};
use super::Engine;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListEnginesResponse {
    pub data: Vec<Engine>,
    pub object: String,
}
impl std::fmt::Display for ListEnginesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}