use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::item_pocket::{ItemPocket, ItemPocketId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPocketData {
    id: ItemPocketId,
    identifier: String,
}

impl PokeApiModel for ItemPocketData {
    fn file_name() -> &'static str {
        "item_pockets"
    }
}

impl HasId for ItemPocketData {
    type Id = ItemPocketId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<ItemPocket> for ItemPocketData {
    fn into_model(self, data: &RawData) -> ItemPocket {
        ItemPocket {
            id: self.id,
            identifier: self.identifier,
            names: data.item_pocket_names.get_model(&self.id, data),
        }
    }
}
