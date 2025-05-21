use crate::models::localized_effects::LocalizedEffects;
use crate::models::localized_names::LocalizedStrings;
use crate::models::version_group::VersionGroupId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type PokemonMoveEffectId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonMoveEffect {
    pub id: PokemonMoveEffectId,
    pub effects: LocalizedEffects,
    pub effect_changelog: HashMap<VersionGroupId, LocalizedStrings>,
}
