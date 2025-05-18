use crate::models::localized_effects::LocalizedEffects;
use serde::{Deserialize, Serialize};

pub type ItemFlingEffectId = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemFlingEffect {
    pub id: ItemFlingEffectId,
    pub identifier: String,
    pub effects: LocalizedEffects,
}
