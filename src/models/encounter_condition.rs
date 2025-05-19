use crate::models::localized_names::LocalizedNames;
use serde::{Deserialize, Serialize};

pub type EncounterConditionId = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterCondition {
    pub id: EncounterConditionId,
    pub identifier: String,
    pub names: LocalizedNames,
}
