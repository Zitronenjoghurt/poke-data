use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::pokemon_move_method::PokemonMoveMethodId;
use poke_data::models::version_group::VersionGroupId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionGroupMoveMethodData {
    pub version_group_id: VersionGroupId,
    pub pokemon_move_method_id: PokemonMoveMethodId,
}

impl PokeApiModel for VersionGroupMoveMethodData {
    fn file_name() -> &'static str {
        "version_group_pokemon_move_methods"
    }
}

impl HasId for VersionGroupMoveMethodData {
    type Id = VersionGroupId;

    fn id(&self) -> Self::Id {
        self.version_group_id
    }
}

impl IntoModel<Vec<PokemonMoveMethodId>> for Vec<VersionGroupMoveMethodData> {
    fn into_model(self, _data: &RawData) -> Vec<PokemonMoveMethodId> {
        self.into_iter().map(|v| v.pokemon_move_method_id).collect()
    }
}
