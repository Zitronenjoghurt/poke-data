use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::pokemon_move::PokemonMoveId;
use poke_data::types::stat::Stat;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveMetaStatChangeData {
    move_id: PokemonMoveId,
    stat_id: u8,
    change: i8,
}

impl PokeApiModel for MoveMetaStatChangeData {
    fn file_name() -> &'static str {
        "move_meta_stat_changes"
    }
}

impl HasId for MoveMetaStatChangeData {
    type Id = PokemonMoveId;

    fn id(&self) -> Self::Id {
        self.move_id
    }
}

impl IntoModel<HashMap<Stat, i8>> for Vec<MoveMetaStatChangeData> {
    fn into_model(self, _data: &RawData) -> HashMap<Stat, i8> {
        self.iter()
            .map(|entry| (Stat::from(entry.stat_id), entry.change))
            .collect()
    }
}
