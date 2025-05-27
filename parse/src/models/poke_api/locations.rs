use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::location::{LocationId, UnlinkedLocation};
use poke_data::models::region::RegionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationData {
    id: LocationId,
    identifier: String,
    region_id: Option<RegionId>,
}

impl PokeApiModel for LocationData {
    fn file_name() -> &'static str {
        "locations"
    }
}

impl HasId for LocationData {
    type Id = LocationId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<UnlinkedLocation> for LocationData {
    fn into_model(self, data: &RawData) -> UnlinkedLocation {
        UnlinkedLocation {
            id: self.id,
            identifier: self.identifier,
            region_id: self.region_id,
            names: data.location_names.get_model(&self.id, data),
            game_indices: data.location_game_indices.get_model(&self.id, data),
        }
    }
}
