
use serde::{Serialize, Deserialize};
use super::PetTag;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Pet {
    pub id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<PetTag>,
}
impl std::fmt::Display for Pet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}