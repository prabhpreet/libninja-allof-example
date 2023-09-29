
use serde::{Serialize, Deserialize};
use super::Pet;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pets(pub Vec<Pet>);