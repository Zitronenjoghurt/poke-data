use crate::models::localized_names::LocalizedStrings;
use serde::{Deserialize, Serialize};

pub type PokemonMoveCategoryId = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonMoveCategory {
    pub id: PokemonMoveCategoryId,
    pub identifier: String,
    pub description: LocalizedStrings,
}
