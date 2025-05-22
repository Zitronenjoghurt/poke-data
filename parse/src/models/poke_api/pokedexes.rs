use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::pokedex::{PokedexId, UnlinkedPokedex};
use poke_data::models::region::RegionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokedexData {
    id: PokedexId,
    region_id: Option<RegionId>,
    identifier: String,
    is_main_series: u8,
}

impl PokeApiModel for PokedexData {
    fn file_name() -> &'static str {
        "pokedexes"
    }
}

impl HasId for PokedexData {
    type Id = PokedexId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<UnlinkedPokedex> for PokedexData {
    fn into_model(self, data: &RawData) -> UnlinkedPokedex {
        UnlinkedPokedex {
            id: self.id,
            identifier: self.identifier,
            prose: data.pokedex_prose.get_model(&self.id, data),
            region_id: self.region_id,
            is_main_series: self.is_main_series == 1,
            version_group_ids: data.pokedex_version_groups.get_model(&self.id, data),
            species_map: data.pokemon_dex_numbers.get_model(&self.id, data),
        }
    }
}
