use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::contest_effect::{ContestEffect, ContestEffectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContestEffectData {
    id: ContestEffectId,
    appeal: u8,
    jam: u8,
}

impl PokeApiModel for ContestEffectData {
    fn file_name() -> &'static str {
        "contest_effects"
    }
}

impl HasId for ContestEffectData {
    type Id = ContestEffectId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<ContestEffect> for ContestEffectData {
    fn into_model(self, data: &RawData) -> ContestEffect {
        ContestEffect {
            id: self.id,
            appeal: self.appeal,
            jam: self.jam,
            prose: data.contest_effect_prose.get_model(&self.id, data),
        }
    }
}
