use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedString;
use poke_data::models::encounter_condition::EncounterConditionId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterConditionProseData {
    encounter_condition_id: EncounterConditionId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for EncounterConditionProseData {
    fn file_name() -> &'static str {
        "encounter_condition_prose"
    }
}

impl HasId for EncounterConditionProseData {
    type Id = EncounterConditionId;

    fn id(&self) -> Self::Id {
        self.encounter_condition_id
    }
}

impl HasLocalizedString for EncounterConditionProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn string(&self) -> String {
        self.name.clone()
    }
}
