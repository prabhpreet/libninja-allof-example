
use serde::{Serialize, Deserialize};
use super::{OwnerInfo, PetHealth};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetTag {
    #[serde(flatten)]
    pub owner_info: OwnerInfo,
    #[serde(flatten)]
    pub pet_health: PetHealth,
    #[serde(rename = "eye-color")]
    pub eye_color: String,
    pub weight: i64,
}
impl std::fmt::Display for PetTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PetTag {
    type Target = OwnerInfo;
    fn deref(&self) -> &Self::Target {
        &self.owner_info
    }
}
impl std::ops::DerefMut for PetTag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.owner_info
    }
}