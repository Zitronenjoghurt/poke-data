use crate::models::localized_names::LocalizedNames;
use serde::{Deserialize, Serialize};

pub type HabitatId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Habitat {
    pub id: HabitatId,
    pub identifier: String,
    pub names: LocalizedNames,
}
