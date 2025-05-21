use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedString;
use poke_data::models::encounter_condition_value::EncounterConditionValueId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterConditionValueProseData {
    encounter_condition_value_id: EncounterConditionValueId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for EncounterConditionValueProseData {
    fn file_name() -> &'static str {
        "encounter_condition_value_prose"
    }
}

impl HasId for EncounterConditionValueProseData {
    type Id = EncounterConditionValueId;

    fn id(&self) -> Self::Id {
        self.encounter_condition_value_id
    }
}

impl HasLocalizedString for EncounterConditionValueProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn string(&self) -> String {
        self.name.clone()
    }
}
