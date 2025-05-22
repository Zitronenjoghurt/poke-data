use crate::models::localized_name_descriptions::LocalizedNameDescriptions;
use serde::{Deserialize, Serialize};

pub type PokemonMoveMethodId = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonMoveMethod {
    pub id: PokemonMoveMethodId,
    pub identifier: String,
    pub prose: LocalizedNameDescriptions,
}
