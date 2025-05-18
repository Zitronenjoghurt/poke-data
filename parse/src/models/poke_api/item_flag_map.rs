use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::item::ItemId;
use poke_data::models::item_flag::ItemFlagId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemFlagMapData {
    item_id: ItemId,
    item_flag_id: ItemFlagId,
}

impl PokeApiModel for ItemFlagMapData {
    fn file_name() -> &'static str {
        "item_flag_map"
    }
}

impl HasId for ItemFlagMapData {
    type Id = ItemId;

    fn id(&self) -> Self::Id {
        self.item_id
    }
}

impl IntoModel<Vec<ItemFlagId>> for Vec<ItemFlagMapData> {
    fn into_model(self, _data: &RawData) -> Vec<ItemFlagId> {
        self.iter().map(|item| item.item_flag_id).collect()
    }
}
