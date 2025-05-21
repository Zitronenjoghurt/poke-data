use crate::models::localized_names::LocalizedStrings;
use serde::{Deserialize, Serialize};

pub type SuperContestEffectId = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperContestEffect {
    pub id: SuperContestEffectId,
    pub appeal: u8,
    pub flavor_texts: LocalizedStrings,
}
