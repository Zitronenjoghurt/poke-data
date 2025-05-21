use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_effects::HasLocalizedEffects;
use poke_data::models::language::LanguageId;
use poke_data::models::pokemon_move_effect::PokemonMoveEffectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveEffectProseData {
    move_effect_id: PokemonMoveEffectId,
    local_language_id: LanguageId,
    effect: String,
    short_effect: String,
}

impl PokeApiModel for MoveEffectProseData {
    fn file_name() -> &'static str {
        "move_effect_prose"
    }
}

impl HasId for MoveEffectProseData {
    type Id = PokemonMoveEffectId;

    fn id(&self) -> Self::Id {
        self.move_effect_id
    }
}

impl HasLocalizedEffects for MoveEffectProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn effect(&self) -> String {
        self.effect.clone()
    }

    fn short_effect(&self) -> String {
        self.short_effect.clone()
    }
}
