use crate::models::localized_name_descriptions::LocalizedNameDescriptions;
use serde::{Deserialize, Serialize};

pub type PokemonMoveTargetId = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonMoveTarget {
    pub id: PokemonMoveTargetId,
    pub identifier: String,
    pub prose: LocalizedNameDescriptions,
}
