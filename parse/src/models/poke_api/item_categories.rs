use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::item_category::{ItemCategoryId, UnlinkedItemCategory};
use poke_data::models::item_pocket::ItemPocketId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCategoryData {
    id: ItemCategoryId,
    identifier: String,
    pocket_id: ItemPocketId,
}

impl PokeApiModel for ItemCategoryData {
    fn file_name() -> &'static str {
        "item_categories"
    }
}

impl HasId for ItemCategoryData {
    type Id = ItemCategoryId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<UnlinkedItemCategory> for ItemCategoryData {
    fn into_model(self, data: &RawData) -> UnlinkedItemCategory {
        UnlinkedItemCategory {
            id: self.id,
            identifier: self.identifier,
            names: data.item_category_prose.get_model(&self.id, data),
            pocket_id: self.pocket_id,
        }
    }
}
