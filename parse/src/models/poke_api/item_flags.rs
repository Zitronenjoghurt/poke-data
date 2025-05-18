use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::item_flag::{ItemFlag, ItemFlagId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemFlagData {
    id: ItemFlagId,
    identifier: String,
}

impl PokeApiModel for ItemFlagData {
    fn file_name() -> &'static str {
        "item_flags"
    }
}

impl HasId for ItemFlagData {
    type Id = ItemFlagId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<ItemFlag> for ItemFlagData {
    fn into_model(self, data: &RawData) -> ItemFlag {
        ItemFlag {
            id: self.id,
            identifier: self.identifier,
            prose: data.item_flag_prose.get_model(&self.id, data),
        }
    }
}
