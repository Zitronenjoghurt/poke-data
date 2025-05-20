use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedName;
use poke_data::models::language::LanguageId;
use poke_data::models::species::SpeciesId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonSpeciesNameData {
    pokemon_species_id: SpeciesId,
    local_language_id: LanguageId,
    name: String,
    // ToDo: Genus
}

impl PokeApiModel for PokemonSpeciesNameData {
    fn file_name() -> &'static str {
        "pokemon_species_names"
    }
}

impl HasId for PokemonSpeciesNameData {
    type Id = SpeciesId;

    fn id(&self) -> Self::Id {
        self.pokemon_species_id
    }
}

impl HasLocalizedName for PokemonSpeciesNameData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
