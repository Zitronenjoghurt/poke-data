use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_effects::HasLocalizedEffects;
use poke_data::models::ability::AbilityId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityProseData {
    ability_id: AbilityId,
    local_language_id: LanguageId,
    short_effect: String,
    effect: String,
}

impl PokeApiModel for AbilityProseData {
    fn file_name() -> &'static str {
        "ability_prose"
    }
}

impl HasId for AbilityProseData {
    type Id = AbilityId;

    fn id(&self) -> Self::Id {
        self.ability_id
    }
}

impl HasLocalizedEffects for AbilityProseData {
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
