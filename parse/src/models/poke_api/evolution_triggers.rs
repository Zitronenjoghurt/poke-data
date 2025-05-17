use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::evolution_trigger::{EvolutionTrigger, EvolutionTriggerId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionTriggerData {
    id: EvolutionTriggerId,
    identifier: String,
}

impl PokeApiModel for EvolutionTriggerData {
    fn file_name() -> &'static str {
        "evolution_triggers"
    }
}

impl HasId for EvolutionTriggerData {
    type Id = EvolutionTriggerId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<EvolutionTrigger> for EvolutionTriggerData {
    fn into_model(self, data: &RawData) -> EvolutionTrigger {
        EvolutionTrigger {
            id: self.id,
            identifier: self.identifier,
            names: data.evolution_trigger_prose.get_model(&self.id, data),
        }
    }
}
