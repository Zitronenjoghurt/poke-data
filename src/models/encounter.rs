use crate::data::linkable::Linkable;
use crate::data::PokeData;
use crate::models::encounter_condition_value::{
    EncounterConditionValue, EncounterConditionValueId,
};
use crate::models::encounter_method::{EncounterMethod, EncounterMethodId};
use crate::models::location_area::LocationAreaId;
use crate::models::pokemon::PokemonId;
use crate::models::version::{Version, VersionId};
use crate::models::version_group::{VersionGroup, VersionGroupId};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type EncounterId = u32;

#[derive(Debug)]
pub struct Encounter {
    pub id: EncounterId,
    pub location_area_id: LocationAreaId,
    pub pokemon_id: PokemonId,
    pub encounter_method: Arc<EncounterMethod>,
    pub version: Arc<Version>,
    pub version_group: Arc<VersionGroup>,
    pub encounter_condition_values: Vec<Arc<EncounterConditionValue>>,
    pub slot: Option<u8>,
    pub rarity: u8,
    pub min_level: u8,
    pub max_level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedEncounter {
    pub id: EncounterId,
    pub location_area_id: LocationAreaId,
    pub pokemon_id: PokemonId,
    pub encounter_method_id: EncounterMethodId,
    pub version_id: VersionId,
    pub version_group_id: VersionGroupId,
    pub encounter_condition_value_ids: Vec<EncounterConditionValueId>,
    pub slot: Option<u8>,
    pub rarity: u8,
    pub min_level: u8,
    pub max_level: u8,
}

impl Linkable for UnlinkedEncounter {
    type Linked = Arc<Encounter>;

    fn link(&self, data: &PokeData) -> Self::Linked {
        let encounter_method = data
            .encounter_methods
            .get(&self.encounter_method_id)
            .unwrap_or_else(|| {
                panic!(
                    "No encounter method '{}' found for encounter '{}'",
                    self.encounter_method_id, self.id
                )
            })
            .clone();

        let version = data
            .versions
            .get(&self.version_id)
            .unwrap_or_else(|| {
                panic!(
                    "No version '{}' found for encounter '{}'",
                    self.version_id, self.id
                )
            })
            .clone();

        let version_group = data
            .version_groups
            .get(&self.version_group_id)
            .unwrap_or_else(|| {
                panic!(
                    "No version group '{}' found for encounter '{}'",
                    self.version_group_id, self.id
                )
            })
            .clone();

        let encounter_condition_values = self
            .encounter_condition_value_ids
            .iter()
            .map(|id| {
                data.encounter_condition_values
                    .get(id)
                    .unwrap_or_else(|| {
                        panic!(
                            "No encounter condition value '{}' found for encounter '{}'",
                            id, self.id
                        )
                    })
                    .clone()
            })
            .collect();

        let encounter = Encounter {
            id: self.id,
            location_area_id: self.location_area_id,
            pokemon_id: self.pokemon_id,
            encounter_method,
            version,
            version_group,
            encounter_condition_values,
            slot: self.slot,
            rarity: self.rarity,
            min_level: self.min_level,
            max_level: self.max_level,
        };

        Arc::new(encounter)
    }
}
