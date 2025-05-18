use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::item_fling_effect::{ItemFlingEffect, ItemFlingEffectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemFlingEffectData {
    id: ItemFlingEffectId,
    identifier: String,
}

impl PokeApiModel for ItemFlingEffectData {
    fn file_name() -> &'static str {
        "item_fling_effects"
    }
}

impl HasId for ItemFlingEffectData {
    type Id = ItemFlingEffectId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<ItemFlingEffect> for ItemFlingEffectData {
    fn into_model(self, data: &RawData) -> ItemFlingEffect {
        ItemFlingEffect {
            id: self.id,
            identifier: self.identifier,
            effects: data.item_fling_effect_prose.get_model(&self.id, data),
        }
    }
}
