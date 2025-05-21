use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedString;
use poke_data::models::language::LanguageId;
use poke_data::models::pokemon_move_ailment::PokemonMoveAilmentId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveAilmentNameData {
    move_meta_ailment_id: PokemonMoveAilmentId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for MoveAilmentNameData {
    fn file_name() -> &'static str {
        "move_meta_ailment_names"
    }
}

impl HasId for MoveAilmentNameData {
    type Id = PokemonMoveAilmentId;

    fn id(&self) -> Self::Id {
        self.move_meta_ailment_id
    }
}

impl HasLocalizedString for MoveAilmentNameData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn string(&self) -> String {
        self.name.clone()
    }
}
