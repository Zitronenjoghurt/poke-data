use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedString;
use poke_data::models::language::LanguageId;
use poke_data::models::pokemon_move::PokemonMoveId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveNameData {
    move_id: PokemonMoveId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for MoveNameData {
    fn file_name() -> &'static str {
        "move_names"
    }
}

impl HasId for MoveNameData {
    type Id = PokemonMoveId;

    fn id(&self) -> Self::Id {
        self.move_id
    }
}

impl HasLocalizedString for MoveNameData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn string(&self) -> String {
        self.name.clone()
    }
}
