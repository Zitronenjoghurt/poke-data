use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_effects::HasLocalizedEffects;
use poke_data::models::item_fling_effect::ItemFlingEffectId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemFlingEffectProseData {
    item_fling_effect_id: ItemFlingEffectId,
    local_language_id: LanguageId,
    effect: String,
}

impl PokeApiModel for ItemFlingEffectProseData {
    fn file_name() -> &'static str {
        "item_fling_effect_prose"
    }
}

impl HasId for ItemFlingEffectProseData {
    type Id = ItemFlingEffectId;

    fn id(&self) -> Self::Id {
        self.item_fling_effect_id
    }
}

impl HasLocalizedEffects for ItemFlingEffectProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn effect(&self) -> String {
        self.effect.clone()
    }

    fn short_effect(&self) -> String {
        self.effect.clone()
    }
}
