use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::language::LanguageId;
use poke_data::models::species::{SpeciesFlavorText, SpeciesFlavorTexts, SpeciesId};
use poke_data::models::version::VersionId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonSpeciesFlavorTextData {
    species_id: SpeciesId,
    version_id: VersionId,
    language_id: LanguageId,
    flavor_text: String,
}

impl PokeApiModel for PokemonSpeciesFlavorTextData {
    fn file_name() -> &'static str {
        "pokemon_species_flavor_text"
    }
}

impl HasId for PokemonSpeciesFlavorTextData {
    type Id = SpeciesId;

    fn id(&self) -> Self::Id {
        self.species_id
    }
}

impl IntoModel<SpeciesFlavorText> for PokemonSpeciesFlavorTextData {
    fn into_model(self, _data: &RawData) -> SpeciesFlavorText {
        SpeciesFlavorText {
            version_id: self.version_id,
            flavor_text: self.flavor_text.clone(),
        }
    }
}

impl IntoModel<SpeciesFlavorTexts> for Vec<PokemonSpeciesFlavorTextData> {
    fn into_model(self, data: &RawData) -> SpeciesFlavorTexts {
        let texts = self.into_iter().fold(
            HashMap::new(),
            |mut map: HashMap<LanguageId, Vec<SpeciesFlavorText>>, entry| {
                map.entry(entry.language_id)
                    .or_default()
                    .push(entry.clone().into_model(data));
                map
            },
        );
        SpeciesFlavorTexts::new(texts)
    }
}
