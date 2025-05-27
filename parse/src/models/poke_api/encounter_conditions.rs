use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::encounter_condition::{EncounterCondition, EncounterConditionId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterConditionData {
    id: EncounterConditionId,
    identifier: String,
}

impl PokeApiModel for EncounterConditionData {
    fn file_name() -> &'static str {
        "encounter_conditions"
    }
}

impl HasId for EncounterConditionData {
    type Id = EncounterConditionId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<EncounterCondition> for EncounterConditionData {
    fn into_model(self, data: &RawData) -> EncounterCondition {
        EncounterCondition {
            id: self.id,
            identifier: self.identifier,
            names: data.encounter_condition_prose.get_model(&self.id, data),
        }
    }
}
