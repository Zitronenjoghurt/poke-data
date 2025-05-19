use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use poke_data::models::encounter::EncounterId;
use poke_data::models::encounter_condition_value::EncounterConditionValueId;
use serde::{Deserialize, Serialize};

pub type EncounterSlotId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterConditionValueMapData {
    pub encounter_id: EncounterId,
    pub encounter_condition_value_id: EncounterConditionValueId,
}

impl PokeApiModel for EncounterConditionValueMapData {
    fn file_name() -> &'static str {
        "encounter_condition_value_map"
    }
}

impl HasId for EncounterConditionValueMapData {
    type Id = EncounterId;

    fn id(&self) -> Self::Id {
        self.encounter_id
    }
}
