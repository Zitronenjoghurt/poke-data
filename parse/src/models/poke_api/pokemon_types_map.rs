use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::pokemon::PokemonId;
use poke_data::models::pokemon_type::PokemonTypeId;
use poke_data::types::pokemon_type::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonTypeMapData {
    pokemon_id: PokemonId,
    type_id: PokemonTypeId,
    slot: u8,
}

impl PokeApiModel for PokemonTypeMapData {
    fn file_name() -> &'static str {
        "pokemon_types"
    }
}

impl HasId for PokemonTypeMapData {
    type Id = PokemonId;

    fn id(&self) -> Self::Id {
        self.pokemon_id
    }
}

impl IntoModel<Vec<Type>> for Vec<PokemonTypeMapData> {
    fn into_model(self, _data: &RawData) -> Vec<Type> {
        let mut types_sorted = self.clone();
        types_sorted.sort_by(|a, b| a.slot.cmp(&b.slot));
        types_sorted
            .into_iter()
            .map(|type_entry| Type::from(type_entry.type_id))
            .collect()
    }
}
