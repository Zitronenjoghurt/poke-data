use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::ability::{AbilityId, UnlinkedAbility};
use poke_data::models::generation::GenerationId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityData {
    id: AbilityId,
    identifier: String,
    generation_id: GenerationId,
    is_main_series: u8,
}

impl PokeApiModel for AbilityData {
    fn file_name() -> &'static str {
        "abilities"
    }
}

impl HasId for AbilityData {
    type Id = AbilityId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<UnlinkedAbility> for AbilityData {
    fn into_model(self, data: &RawData) -> UnlinkedAbility {
        UnlinkedAbility {
            id: self.id,
            identifier: self.identifier,
            names: data.ability_names.get_model(&self.id, data),
            flavor_texts: data.ability_flavor_texts.get_model(&self.id, data),
            effects: data.ability_prose.get_model(&self.id, data),
            changelog: data.ability_changelogs.get_model(&self.id, data),
            generation_id: self.generation_id,
            is_main_series: self.is_main_series == 1,
        }
    }
}
