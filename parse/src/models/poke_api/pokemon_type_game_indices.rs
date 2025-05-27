use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::generation::GenerationId;
use poke_data::models::pokemon_type::{PokemonTypeGameIndices, PokemonTypeId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonTypeGameIndexData {
    pub type_id: PokemonTypeId,
    pub generation_id: GenerationId,
    pub game_index: u16,
}

impl PokeApiModel for PokemonTypeGameIndexData {
    fn file_name() -> &'static str {
        "type_game_indices"
    }
}

impl HasId for PokemonTypeGameIndexData {
    type Id = PokemonTypeId;

    fn id(&self) -> Self::Id {
        self.type_id
    }
}

impl IntoModel<PokemonTypeGameIndices> for Vec<PokemonTypeGameIndexData> {
    fn into_model(self, _data: &RawData) -> PokemonTypeGameIndices {
        let indices = self
            .iter()
            .map(|index| (index.generation_id, index.game_index))
            .collect();
        PokemonTypeGameIndices::new(indices)
    }
}
