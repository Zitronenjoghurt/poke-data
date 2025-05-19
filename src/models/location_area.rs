use crate::data::linkable::Linkable;
use crate::data::PokeData;
use crate::models::encounter_method::EncounterMethodId;
use crate::models::localized_names::LocalizedNames;
use crate::models::location::{Location, LocationId};
use crate::models::version::VersionId;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type LocationAreaId = u16;

#[derive(Debug)]
pub struct LocationArea {
    pub id: LocationAreaId,
    pub identifier: Option<String>,
    pub names: LocalizedNames,
    pub location: Arc<Location>,
    pub game_index: u16,
    pub encounter_rates: LocationAreaEncounterRates,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationAreaEncounterRates(Vec<LocationAreaEncounterRate>);

impl LocationAreaEncounterRates {
    pub fn new(rates: Vec<LocationAreaEncounterRate>) -> Self {
        Self(rates)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationAreaEncounterRate {
    pub encounter_method_id: EncounterMethodId,
    pub version_id: VersionId,
    pub rate: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedLocationArea {
    pub id: LocationAreaId,
    pub identifier: Option<String>,
    pub names: LocalizedNames,
    pub location_id: LocationId,
    pub game_index: u16,
    pub encounter_rates: LocationAreaEncounterRates,
}

impl Linkable for UnlinkedLocationArea {
    type Linked = Arc<LocationArea>;

    fn link(&self, data: &PokeData) -> Self::Linked {
        let location = data
            .locations
            .get(&self.location_id)
            .unwrap_or_else(|| {
                panic!(
                    "No location '{}' found for location area '{}'",
                    self.location_id, self.id
                )
            })
            .clone();
        let location_area = LocationArea {
            id: self.id,
            identifier: self.identifier.clone(),
            names: self.names.clone(),
            location,
            game_index: self.game_index,
            encounter_rates: self.encounter_rates.clone(),
        };
        Arc::new(location_area)
    }
}
