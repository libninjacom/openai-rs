
use serde::{Serialize, Deserialize};
use super::OpenAiFile;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListFilesResponse {
    pub data: Vec<OpenAiFile>,
    pub object: String,
}
impl std::fmt::Display for ListFilesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}