use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::egg_group::EggGroupId;
use poke_data::models::species::SpeciesId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonEggGroupData {
    species_id: SpeciesId,
    egg_group_id: EggGroupId,
}

impl PokeApiModel for PokemonEggGroupData {
    fn file_name() -> &'static str {
        "pokemon_egg_groups"
    }
}

impl HasId for PokemonEggGroupData {
    type Id = SpeciesId;

    fn id(&self) -> Self::Id {
        self.species_id
    }
}

impl IntoModel<EggGroupId> for PokemonEggGroupData {
    fn into_model(self, _data: &RawData) -> EggGroupId {
        self.egg_group_id
    }
}
