use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedString;
use poke_data::models::color::ColorId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorNameData {
    pokemon_color_id: ColorId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for ColorNameData {
    fn file_name() -> &'static str {
        "pokemon_color_names"
    }
}

impl HasId for ColorNameData {
    type Id = ColorId;

    fn id(&self) -> Self::Id {
        self.pokemon_color_id
    }
}

impl HasLocalizedString for ColorNameData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn string(&self) -> String {
        self.name.clone()
    }
}
