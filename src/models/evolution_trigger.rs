use crate::models::localized_names::LocalizedStrings;
use serde::{Deserialize, Serialize};

pub type EvolutionTriggerId = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionTrigger {
    pub id: EvolutionTriggerId,
    pub identifier: String,
    pub names: LocalizedStrings,
}
