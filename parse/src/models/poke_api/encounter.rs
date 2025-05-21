use crate::models::poke_api::encounter_slots::EncounterSlotId;
use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::encounter::{EncounterId, UnlinkedEncounter};
use poke_data::models::location_area::LocationAreaId;
use poke_data::models::pokemon::PokemonId;
use poke_data::models::version::VersionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterData {
    id: EncounterId,
    version_id: VersionId,
    location_area_id: LocationAreaId,
    encounter_slot_id: EncounterSlotId,
    pokemon_id: PokemonId,
    min_level: u8,
    max_level: u8,
}

impl PokeApiModel for EncounterData {
    fn file_name() -> &'static str {
        "encounters"
    }
}

impl HasId for EncounterData {
    type Id = EncounterId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<UnlinkedEncounter> for EncounterData {
    fn into_model(self, data: &RawData) -> UnlinkedEncounter {
        let encounter_slot = data
            .encounter_slots
            .get(&self.encounter_slot_id)
            .unwrap_or_else(|| {
                panic!(
                    "Encounter slot '{}' not found for encounter '{}'",
                    self.encounter_slot_id, self.id
                )
            });

        let encounter_condition_value_ids = data
            .encounter_condition_value_map
            .get(&self.id)
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|map_entry| map_entry.encounter_condition_value_id)
            .collect();

        UnlinkedEncounter {
            id: self.id,
            location_area_id: self.location_area_id,
            pokemon_id: self.pokemon_id,
            encounter_method_id: encounter_slot.encounter_method_id,
            version_id: self.version_id,
            version_group_id: encounter_slot.version_group_id,
            encounter_condition_value_ids,
            slot: encounter_slot.slot,
            rarity: encounter_slot.rarity,
            min_level: self.min_level,
            max_level: self.max_level,
        }
    }
}
