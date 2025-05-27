use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::pokedex::PokedexId;
use poke_data::models::version_group::VersionGroupId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokedexVersionGroupData {
    pub pokedex_id: PokedexId,
    pub version_group_id: VersionGroupId,
}

impl PokeApiModel for PokedexVersionGroupData {
    fn file_name() -> &'static str {
        "pokedex_version_groups"
    }
}

impl HasId for PokedexVersionGroupData {
    type Id = PokedexId;

    fn id(&self) -> Self::Id {
        self.pokedex_id
    }
}

impl IntoModel<Vec<VersionGroupId>> for Vec<PokedexVersionGroupData> {
    fn into_model(self, _data: &RawData) -> Vec<VersionGroupId> {
        self.into_iter().map(|v| v.version_group_id).collect()
    }
}
