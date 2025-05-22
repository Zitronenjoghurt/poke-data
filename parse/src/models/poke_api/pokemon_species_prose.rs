use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedString;
use poke_data::models::language::LanguageId;
use poke_data::models::species::SpeciesId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonSpeciesProseData {
    pokemon_species_id: SpeciesId,
    local_language_id: LanguageId,
    form_description: String,
}

impl PokeApiModel for PokemonSpeciesProseData {
    fn file_name() -> &'static str {
        "pokemon_species_prose"
    }
}

impl HasId for PokemonSpeciesProseData {
    type Id = SpeciesId;

    fn id(&self) -> Self::Id {
        self.pokemon_species_id
    }
}

impl HasLocalizedString for PokemonSpeciesProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn string(&self) -> String {
        self.form_description.clone()
    }
}
