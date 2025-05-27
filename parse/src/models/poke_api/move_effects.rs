use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::pokemon_move_effect::{PokemonMoveEffect, PokemonMoveEffectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveEffectData {
    id: PokemonMoveEffectId,
}

impl PokeApiModel for MoveEffectData {
    fn file_name() -> &'static str {
        "move_effects"
    }
}

impl HasId for MoveEffectData {
    type Id = PokemonMoveEffectId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<PokemonMoveEffect> for MoveEffectData {
    fn into_model(self, data: &RawData) -> PokemonMoveEffect {
        PokemonMoveEffect {
            id: self.id,
            effects: data.move_effect_prose.get_model(&self.id, data),
            effect_changelog: data.move_effect_changelogs.get_model(&self.id, data),
        }
    }
}
