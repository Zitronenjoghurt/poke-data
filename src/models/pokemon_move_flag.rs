use crate::models::localized_name_descriptions::LocalizedNameDescriptions;
use serde::{Deserialize, Serialize};

pub type PokemonMoveFlagId = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonMoveFlag {
    pub id: PokemonMoveFlagId,
    pub identifier: String,
    pub prose: LocalizedNameDescriptions,
}
