use crate::models::localized_names::LocalizedStrings;
use serde::{Deserialize, Serialize};

pub type EncounterMethodId = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterMethod {
    pub id: EncounterMethodId,
    pub identifier: String,
    pub names: LocalizedStrings,
    pub order: u8,
}
