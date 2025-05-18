use crate::models::localized_names::LocalizedNames;
use serde::{Deserialize, Serialize};

pub type EncounterMethodId = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterMethod {
    pub id: EncounterMethodId,
    pub identifier: String,
    pub names: LocalizedNames,
    pub order: u8,
}
