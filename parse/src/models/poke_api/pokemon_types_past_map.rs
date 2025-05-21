use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::generation::GenerationId;
use poke_data::models::pokemon::PokemonId;
use poke_data::models::pokemon_type::PokemonTypeId;
use poke_data::types::pokemon_type::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonTypePastMapData {
    pokemon_id: PokemonId,
    generation_id: GenerationId,
    type_id: PokemonTypeId,
    slot: u8,
}

impl PokeApiModel for PokemonTypePastMapData {
    fn file_name() -> &'static str {
        "pokemon_types_past"
    }
}

impl HasId for PokemonTypePastMapData {
    type Id = PokemonId;

    fn id(&self) -> Self::Id {
        self.pokemon_id
    }
}

impl IntoModel<HashMap<GenerationId, Vec<Type>>> for Vec<PokemonTypePastMapData> {
    fn into_model(self, _data: &RawData) -> HashMap<GenerationId, Vec<Type>> {
        let mut entries_sorted = self.clone();
        entries_sorted.sort_by(|a, b| a.slot.cmp(&b.slot));

        let mut map = HashMap::new();
        for entry in entries_sorted {
            map.entry(entry.generation_id)
                .or_insert_with(Vec::new)
                .push(entry.type_id.into());
        }

        map
    }
}
