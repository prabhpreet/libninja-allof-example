
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OwnerInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    pub phone: String,
}
impl std::fmt::Display for OwnerInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}