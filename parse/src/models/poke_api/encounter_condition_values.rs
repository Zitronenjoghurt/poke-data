use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::encounter_condition::EncounterConditionId;
use poke_data::models::encounter_condition_value::{
    EncounterConditionValueId, UnlinkedEncounterConditionValue,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterConditionValueData {
    id: EncounterConditionValueId,
    encounter_condition_id: EncounterConditionId,
    identifier: String,
    is_default: u8,
}

impl PokeApiModel for EncounterConditionValueData {
    fn file_name() -> &'static str {
        "encounter_condition_values"
    }
}

impl HasId for EncounterConditionValueData {
    type Id = EncounterConditionValueId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<UnlinkedEncounterConditionValue> for EncounterConditionValueData {
    fn into_model(self, data: &RawData) -> UnlinkedEncounterConditionValue {
        UnlinkedEncounterConditionValue {
            id: self.id,
            identifier: self.identifier,
            names: data
                .encounter_condition_value_prose
                .get_model(&self.id, data),
            is_default: self.is_default == 1,
            encounter_condition_id: self.encounter_condition_id,
        }
    }
}
