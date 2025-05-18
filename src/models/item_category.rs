use crate::models::item_pocket::{ItemPocket, ItemPocketId};
use crate::models::localized_names::LocalizedNames;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub type ItemCategoryId = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedItemCategory {
    pub id: ItemCategoryId,
    pub identifier: String,
    pub names: LocalizedNames,
    pub pocket_id: ItemPocketId,
}

impl UnlinkedItemCategory {
    pub fn link(&self, pockets: &HashMap<ItemPocketId, Arc<ItemPocket>>) -> Arc<ItemCategory> {
        let category = ItemCategory {
            id: self.id,
            identifier: self.identifier.clone(),
            names: self.names.clone(),
            pocket: pockets.get(&self.pocket_id).unwrap().clone(),
        };
        Arc::new(category)
    }
}

#[derive(Debug)]
pub struct ItemCategory {
    pub id: ItemCategoryId,
    pub identifier: String,
    pub names: LocalizedNames,
    pub pocket: Arc<ItemPocket>,
}
