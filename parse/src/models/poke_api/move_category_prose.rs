use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedString;
use poke_data::models::language::LanguageId;
use poke_data::models::pokemon_move_category::PokemonMoveCategoryId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveCategoryProseData {
    move_meta_category_id: PokemonMoveCategoryId,
    local_language_id: LanguageId,
    description: String,
}

impl PokeApiModel for MoveCategoryProseData {
    fn file_name() -> &'static str {
        "move_meta_category_prose"
    }
}

impl HasId for MoveCategoryProseData {
    type Id = PokemonMoveCategoryId;

    fn id(&self) -> Self::Id {
        self.move_meta_category_id
    }
}

impl HasLocalizedString for MoveCategoryProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn string(&self) -> String {
        self.description.clone()
    }
}
