use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedString;
use poke_data::models::language::LanguageId;
use poke_data::models::pokemon_form::PokemonFormId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonFormNameData {
    pokemon_form_id: PokemonFormId,
    local_language_id: LanguageId,
    form_name: String,
}

impl PokeApiModel for PokemonFormNameData {
    fn file_name() -> &'static str {
        "pokemon_form_names"
    }
}

impl HasId for PokemonFormNameData {
    type Id = PokemonFormId;

    fn id(&self) -> Self::Id {
        self.pokemon_form_id
    }
}

impl HasLocalizedString for PokemonFormNameData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn string(&self) -> String {
        self.form_name.clone()
    }
}
