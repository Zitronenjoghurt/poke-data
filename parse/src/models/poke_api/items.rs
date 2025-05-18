use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::item::{ItemId, UnlinkedItem};
use poke_data::models::item_category::ItemCategoryId;
use poke_data::models::item_fling_effect::ItemFlingEffectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    id: ItemId,
    identifier: String,
    category_id: ItemCategoryId,
    cost: u32,
    fling_power: Option<u8>,
    fling_effect_id: Option<ItemFlingEffectId>,
}

impl PokeApiModel for ItemData {
    fn file_name() -> &'static str {
        "items"
    }
}

impl HasId for ItemData {
    type Id = ItemId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<UnlinkedItem> for ItemData {
    fn into_model(self, data: &RawData) -> UnlinkedItem {
        UnlinkedItem {
            id: self.id,
            identifier: self.identifier,
            names: data.item_names.get_model(&self.id, data),
            flavor_texts: data.item_flavor_texts.get_model(&self.id, data),
            effects: data.item_prose.get_model(&self.id, data),
            category_id: self.category_id,
            cost: self.cost,
            fling_power: self.fling_power,
            fling_effect_id: self.fling_effect_id,
            flag_ids: data.item_flag_map.get_model(&self.id, data),
            game_indices: data.item_game_indices.get_model(&self.id, data),
        }
    }
}
