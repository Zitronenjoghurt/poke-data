use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::generation::GenerationId;
use poke_data::models::location::{LocationGameIndices, LocationId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationGameIndexData {
    pub location_id: LocationId,
    pub generation_id: GenerationId,
    pub game_index: u16,
}

impl PokeApiModel for LocationGameIndexData {
    fn file_name() -> &'static str {
        "location_game_indices"
    }
}

impl HasId for LocationGameIndexData {
    type Id = LocationId;

    fn id(&self) -> Self::Id {
        self.location_id
    }
}

impl IntoModel<LocationGameIndices> for Vec<LocationGameIndexData> {
    fn into_model(self, _data: &RawData) -> LocationGameIndices {
        let indices = self
            .iter()
            .map(|index| (index.generation_id, index.game_index))
            .collect();
        LocationGameIndices::new(indices)
    }
}
