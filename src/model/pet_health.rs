
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PetHealth {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neutered: Option<bool>,
    pub vaccinated: bool,
}
impl std::fmt::Display for PetHealth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}