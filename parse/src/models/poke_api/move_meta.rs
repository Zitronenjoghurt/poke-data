use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::pokemon_move::meta::UnlinkedPokemonMoveMeta;
use poke_data::models::pokemon_move::PokemonMoveId;
use poke_data::models::pokemon_move_ailment::PokemonMoveAilmentId;
use poke_data::models::pokemon_move_category::PokemonMoveCategoryId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveMetaData {
    move_id: PokemonMoveId,
    meta_category_id: PokemonMoveCategoryId,
    meta_ailment_id: PokemonMoveAilmentId,
    min_hits: Option<u8>,
    max_hits: Option<u8>,
    min_turns: Option<u8>,
    max_turns: Option<u8>,
    drain: i8,
    healing: i8,
    crit_rate: u8,
    ailment_chance: u8,
    flinch_chance: u8,
    stat_chance: u8,
}

impl PokeApiModel for MoveMetaData {
    fn file_name() -> &'static str {
        "move_meta"
    }
}

impl HasId for MoveMetaData {
    type Id = PokemonMoveId;

    fn id(&self) -> Self::Id {
        self.move_id
    }
}

impl IntoModel<UnlinkedPokemonMoveMeta> for MoveMetaData {
    fn into_model(self, data: &RawData) -> UnlinkedPokemonMoveMeta {
        UnlinkedPokemonMoveMeta {
            category_id: self.meta_category_id,
            ailment_id: self.meta_ailment_id,
            min_hits: self.min_hits,
            max_hits: self.max_hits,
            min_turns: self.min_turns,
            max_turns: self.max_turns,
            stat_changes: data.move_meta_stat_changes.get_model(&self.move_id, data),
            drain: self.drain,
            healing: self.healing,
            crit_rate: self.crit_rate,
            ailment_chance: self.ailment_chance,
            flinch_chance: self.flinch_chance,
            stat_chance: self.stat_chance,
        }
    }
}
