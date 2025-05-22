use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::pokemon::moveset::{UnlinkedMoveset, UnlinkedMovesetEntry};
use poke_data::models::pokemon::PokemonId;
use poke_data::models::pokemon_move::PokemonMoveId;
use poke_data::models::pokemon_move_method::PokemonMoveMethodId;
use poke_data::models::version_group::VersionGroupId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonMoveMapData {
    pokemon_id: PokemonId,
    version_group_id: VersionGroupId,
    move_id: PokemonMoveId,
    pokemon_move_method_id: PokemonMoveMethodId,
    level: u8,
    order: Option<u8>,
    mastery: Option<u8>,
}

impl PokeApiModel for PokemonMoveMapData {
    fn file_name() -> &'static str {
        "pokemon_moves"
    }
}

impl HasId for PokemonMoveMapData {
    type Id = PokemonId;

    fn id(&self) -> Self::Id {
        self.pokemon_id
    }
}

impl IntoModel<UnlinkedMovesetEntry> for PokemonMoveMapData {
    fn into_model(self, _data: &RawData) -> UnlinkedMovesetEntry {
        UnlinkedMovesetEntry {
            move_id: self.move_id,
            move_method_id: self.pokemon_move_method_id,
            level: self.level,
            order: self.order,
            mastery: self.mastery,
        }
    }
}

impl IntoModel<UnlinkedMoveset> for Vec<PokemonMoveMapData> {
    fn into_model(self, data: &RawData) -> UnlinkedMoveset {
        let entries = self.into_iter().fold(
            HashMap::new(),
            |mut acc: HashMap<VersionGroupId, Vec<UnlinkedMovesetEntry>>, entry| {
                acc.entry(entry.version_group_id)
                    .or_default()
                    .push(entry.into_model(data));
                acc
            },
        );
        UnlinkedMoveset::new(entries)
    }
}
