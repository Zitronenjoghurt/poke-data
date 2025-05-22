use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::pokedex::PokedexId;
use poke_data::models::species::SpeciesId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonDexNumberData {
    species_id: SpeciesId,
    pokedex_id: PokedexId,
    pokedex_number: u16,
}

impl PokeApiModel for PokemonDexNumberData {
    fn file_name() -> &'static str {
        "pokemon_dex_numbers"
    }
}

impl HasId for PokemonDexNumberData {
    type Id = PokedexId;

    fn id(&self) -> Self::Id {
        self.pokedex_id
    }
}

impl IntoModel<BTreeMap<u16, SpeciesId>> for Vec<PokemonDexNumberData> {
    fn into_model(self, _data: &RawData) -> BTreeMap<u16, SpeciesId> {
        self.iter()
            .map(|entry| (entry.pokedex_number, entry.species_id))
            .collect()
    }
}
