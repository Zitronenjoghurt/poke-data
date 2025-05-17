use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_effects::HasLocalizedEffects;
use poke_data::models::item::ItemId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemProseData {
    item_id: ItemId,
    local_language_id: LanguageId,
    short_effect: String,
    effect: String,
}

impl PokeApiModel for ItemProseData {
    fn file_name() -> &'static str {
        "item_prose"
    }
}

impl HasId for ItemProseData {
    type Id = ItemId;

    fn id(&self) -> Self::Id {
        self.item_id
    }
}

impl HasLocalizedEffects for ItemProseData {
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
