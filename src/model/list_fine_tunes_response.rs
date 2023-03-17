
use serde::{Serialize, Deserialize};
use super::FineTune;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListFineTunesResponse {
    pub data: Vec<FineTune>,
    pub object: String,
}
impl std::fmt::Display for ListFineTunesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}