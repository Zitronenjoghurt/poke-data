use crate::data::linkable::Linkable;
use crate::data::PokeData;
use crate::models::item_pocket::{ItemPocket, ItemPocketId};
use crate::models::localized_names::LocalizedNames;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type ItemCategoryId = u8;

#[derive(Debug)]
pub struct ItemCategory {
    pub id: ItemCategoryId,
    pub identifier: String,
    pub names: LocalizedNames,
    pub pocket: Arc<ItemPocket>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedItemCategory {
    pub id: ItemCategoryId,
    pub identifier: String,
    pub names: LocalizedNames,
    pub pocket_id: ItemPocketId,
}

impl Linkable for UnlinkedItemCategory {
    type Linked = Arc<ItemCategory>;

    fn link(&self, data: &PokeData) -> Self::Linked {
        let pocket = data
            .item_pockets
            .get(&self.pocket_id)
            .unwrap_or_else(|| {
                panic!(
                    "No item pocket '{}' found for item category '{}'",
                    self.pocket_id, self.id
                )
            })
            .clone();
        let category = ItemCategory {
            id: self.id,
            identifier: self.identifier.clone(),
            names: self.names.clone(),
            pocket,
        };
        Arc::new(category)
    }
}
