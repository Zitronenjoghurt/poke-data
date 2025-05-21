use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name_description::HasLocalizedNameDescription;
use poke_data::models::language::LanguageId;
use poke_data::models::pokemon_move_target::PokemonMoveTargetId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveTargetProseData {
    move_target_id: PokemonMoveTargetId,
    local_language_id: LanguageId,
    name: String,
    description: String,
}

impl PokeApiModel for MoveTargetProseData {
    fn file_name() -> &'static str {
        "move_target_prose"
    }
}

impl HasId for MoveTargetProseData {
    type Id = PokemonMoveTargetId;

    fn id(&self) -> Self::Id {
        self.move_target_id
    }
}

impl HasLocalizedNameDescription for MoveTargetProseData {
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
