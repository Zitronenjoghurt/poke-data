use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::item_pocket::{ItemPocket, ItemPocketId};
use crate::models::localized_names::LocalizedStrings;
use crate::traits::has_identifier::HasIdentifier;
use crate::traits::has_localized_names::HasLocalizedNames;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type ItemCategoryId = u8;

#[derive(Debug)]
pub struct ItemCategory {
    pub id: ItemCategoryId,
    pub identifier: String,
    pub names: LocalizedStrings,
    pub pocket: Arc<ItemPocket>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedItemCategory {
    pub id: ItemCategoryId,
    pub identifier: String,
    pub names: LocalizedStrings,
    pub pocket_id: ItemPocketId,
}

impl Linkable for UnlinkedItemCategory {
    type Linked = Arc<ItemCategory>;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let pocket = context
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

impl HasLocalizedNames for ItemCategory {
    fn localized_names(&self) -> &LocalizedStrings {
        &self.names
    }
}

impl HasIdentifier for ItemCategory {
    fn identifier(&self) -> &str {
        &self.identifier
    }
}
