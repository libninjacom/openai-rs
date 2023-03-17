
use serde::{Serialize, Deserialize};
use super::FineTuneEvent;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListFineTuneEventsResponse {
    pub data: Vec<FineTuneEvent>,
    pub object: String,
}
impl std::fmt::Display for ListFineTuneEventsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}