use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::berry_firmness::{BerryFirmness, BerryFirmnessId};
use crate::models::item::{Item, ItemId};
use crate::models::localized_names::LocalizedNames;
use crate::traits::has_identifier::HasIdentifier;
use crate::traits::has_localized_names::HasLocalizedNames;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type BerryId = u16;

#[derive(Debug)]
pub struct Berry {
    pub id: BerryId,
    pub item: Arc<Item>,
    pub firmness: Arc<BerryFirmness>,
    pub size: u16,
    pub max_harvest: u8,
    pub growth_time: u8,
    pub soil_dryness: u8,
    pub smoothness: u8,
    // ToDo: Natural Gift Power
    // ToDo: Natural Gift Type
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedBerry {
    pub id: BerryId,
    pub item_id: ItemId,
    pub firmness_id: BerryFirmnessId,
    pub size: u16,
    pub max_harvest: u8,
    pub growth_time: u8,
    pub soil_dryness: u8,
    pub smoothness: u8,
}

impl Linkable for UnlinkedBerry {
    type Linked = Arc<Berry>;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let item = context
            .items
            .get(&self.item_id)
            .unwrap_or_else(|| panic!("No item '{}' found for berry '{}'", self.item_id, self.id))
            .clone();

        let firmness = context
            .berry_firmnesses
            .get(&self.firmness_id)
            .unwrap_or_else(|| {
                panic!(
                    "No firmness '{}' found for berry '{}'",
                    self.firmness_id, self.id
                )
            })
            .clone();

        let berry = Berry {
            id: self.id,
            item,
            firmness,
            size: self.size,
            max_harvest: self.max_harvest,
            growth_time: self.growth_time,
            soil_dryness: self.soil_dryness,
            smoothness: self.smoothness,
        };

        Arc::new(berry)
    }
}

impl HasIdentifier for Berry {
    fn identifier(&self) -> &str {
        &self.item.identifier
    }
}

impl HasLocalizedNames for Berry {
    fn localized_names(&self) -> &LocalizedNames {
        &self.item.names
    }
}
