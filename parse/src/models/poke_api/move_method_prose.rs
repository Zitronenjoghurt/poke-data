use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name_description::HasLocalizedNameDescription;
use poke_data::models::language::LanguageId;
use poke_data::models::pokemon_move_method::PokemonMoveMethodId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveMethodProseData {
    pokemon_move_method_id: PokemonMoveMethodId,
    local_language_id: LanguageId,
    name: String,
    description: String,
}

impl PokeApiModel for MoveMethodProseData {
    fn file_name() -> &'static str {
        "pokemon_move_method_prose"
    }
}

impl HasId for MoveMethodProseData {
    type Id = PokemonMoveMethodId;

    fn id(&self) -> Self::Id {
        self.pokemon_move_method_id
    }
}

impl HasLocalizedNameDescription for MoveMethodProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}
