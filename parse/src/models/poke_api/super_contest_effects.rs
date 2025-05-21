use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::super_contest_effect::{SuperContestEffect, SuperContestEffectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperContestEffectData {
    id: SuperContestEffectId,
    appeal: u8,
}

impl PokeApiModel for SuperContestEffectData {
    fn file_name() -> &'static str {
        "super_contest_effects"
    }
}

impl HasId for SuperContestEffectData {
    type Id = SuperContestEffectId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<SuperContestEffect> for SuperContestEffectData {
    fn into_model(self, data: &RawData) -> SuperContestEffect {
        SuperContestEffect {
            id: self.id,
            appeal: self.appeal,
            flavor_texts: data.super_contest_effect_prose.get_model(&self.id, data),
        }
    }
}
