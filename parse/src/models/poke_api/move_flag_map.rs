use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::pokemon_move::PokemonMoveId;
use poke_data::models::pokemon_move_flag::PokemonMoveFlagId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveFlagMapData {
    move_id: PokemonMoveId,
    move_flag_id: PokemonMoveFlagId,
}

impl PokeApiModel for MoveFlagMapData {
    fn file_name() -> &'static str {
        "move_flag_map"
    }
}

impl HasId for MoveFlagMapData {
    type Id = PokemonMoveId;

    fn id(&self) -> Self::Id {
        self.move_id
    }
}

impl IntoModel<PokemonMoveFlagId> for MoveFlagMapData {
    fn into_model(self, _data: &RawData) -> PokemonMoveFlagId {
        self.move_flag_id
    }
}
