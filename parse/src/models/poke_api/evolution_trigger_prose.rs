use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedName;
use poke_data::models::evolution_trigger::EvolutionTriggerId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionTriggerProseData {
    evolution_trigger_id: EvolutionTriggerId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for EvolutionTriggerProseData {
    fn file_name() -> &'static str {
        "evolution_trigger_prose"
    }
}

impl HasId for EvolutionTriggerProseData {
    type Id = EvolutionTriggerId;

    fn id(&self) -> Self::Id {
        self.evolution_trigger_id
    }
}

impl HasLocalizedName for EvolutionTriggerProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
