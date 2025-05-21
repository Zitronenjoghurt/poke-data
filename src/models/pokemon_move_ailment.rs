use crate::models::localized_names::LocalizedStrings;
use serde::{Deserialize, Serialize};

pub type PokemonMoveAilmentId = i8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonMoveAilment {
    pub id: PokemonMoveAilmentId,
    pub identifier: String,
    pub names: LocalizedStrings,
}
