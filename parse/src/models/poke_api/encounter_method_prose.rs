use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedName;
use poke_data::models::encounter_method::EncounterMethodId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterMethodProseData {
    encounter_method_id: EncounterMethodId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for EncounterMethodProseData {
    fn file_name() -> &'static str {
        "encounter_method_prose"
    }
}

impl HasId for EncounterMethodProseData {
    type Id = EncounterMethodId;

    fn id(&self) -> Self::Id {
        self.encounter_method_id
    }
}

impl HasLocalizedName for EncounterMethodProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
