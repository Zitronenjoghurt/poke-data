use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::location::LocationId;
use poke_data::models::location_area::{LocationAreaId, UnlinkedLocationArea};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationAreaData {
    id: LocationAreaId,
    identifier: Option<String>,
    location_id: LocationId,
    game_index: u16,
}

impl PokeApiModel for LocationAreaData {
    fn file_name() -> &'static str {
        "location_areas"
    }
}

impl HasId for LocationAreaData {
    type Id = LocationAreaId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<UnlinkedLocationArea> for LocationAreaData {
    fn into_model(self, data: &RawData) -> UnlinkedLocationArea {
        UnlinkedLocationArea {
            id: self.id,
            identifier: self.identifier,
            names: data.location_area_prose.get_model(&self.id, data),
            location_id: self.location_id,
            game_index: self.game_index,
            encounter_rates: data.location_area_encounter_rates.get_model(&self.id, data),
        }
    }
}
