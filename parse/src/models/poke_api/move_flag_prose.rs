use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name_description::HasLocalizedNameDescription;
use poke_data::models::language::LanguageId;
use poke_data::models::pokemon_move_flag::PokemonMoveFlagId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveFlagProseData {
    move_flag_id: PokemonMoveFlagId,
    local_language_id: LanguageId,
    name: String,
    description: String,
}

impl PokeApiModel for MoveFlagProseData {
    fn file_name() -> &'static str {
        "move_flag_prose"
    }
}

impl HasId for MoveFlagProseData {
    type Id = PokemonMoveFlagId;

    fn id(&self) -> Self::Id {
        self.move_flag_id
    }
}

impl HasLocalizedNameDescription for MoveFlagProseData {
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
