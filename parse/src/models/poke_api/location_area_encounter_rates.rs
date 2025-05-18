use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::encounter_method::EncounterMethodId;
use poke_data::models::location_area::{
    LocationAreaEncounterRate, LocationAreaEncounterRates, LocationAreaId,
};
use poke_data::models::version::VersionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationAreaEncounterRateData {
    location_area_id: LocationAreaId,
    encounter_method_id: EncounterMethodId,
    version_id: VersionId,
    rate: u8,
}

impl PokeApiModel for LocationAreaEncounterRateData {
    fn file_name() -> &'static str {
        "location_area_encounter_rates"
    }
}

impl HasId for LocationAreaEncounterRateData {
    type Id = LocationAreaId;

    fn id(&self) -> Self::Id {
        self.location_area_id
    }
}

impl IntoModel<LocationAreaEncounterRate> for LocationAreaEncounterRateData {
    fn into_model(self, _data: &RawData) -> LocationAreaEncounterRate {
        LocationAreaEncounterRate {
            encounter_method_id: self.encounter_method_id,
            version_id: self.version_id,
            rate: self.rate,
        }
    }
}

impl IntoModel<LocationAreaEncounterRates> for Vec<LocationAreaEncounterRateData> {
    fn into_model(self, data: &RawData) -> LocationAreaEncounterRates {
        let rates = self
            .iter()
            .map(|rate| rate.clone().into_model(data))
            .collect();
        LocationAreaEncounterRates::new(rates)
    }
}
