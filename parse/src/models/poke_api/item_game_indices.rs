use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::generation::GenerationId;
use poke_data::models::item::{ItemGameIndices, ItemId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemGameIndexData {
    pub item_id: ItemId,
    pub generation_id: GenerationId,
    pub game_index: u16,
}

impl PokeApiModel for ItemGameIndexData {
    fn file_name() -> &'static str {
        "item_game_indices"
    }
}

impl HasId for ItemGameIndexData {
    type Id = ItemId;

    fn id(&self) -> Self::Id {
        self.item_id
    }
}

impl IntoModel<ItemGameIndices> for Vec<ItemGameIndexData> {
    fn into_model(self, _data: &RawData) -> ItemGameIndices {
        let indices = self
            .iter()
            .map(|index| (index.generation_id, index.game_index))
            .collect();
        ItemGameIndices::new(indices)
    }
}
