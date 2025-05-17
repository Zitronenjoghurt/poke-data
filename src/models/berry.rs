use crate::models::berry_firmness::{BerryFirmness, BerryFirmnessId};
use crate::models::item::{Item, ItemId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub type BerryId = u16;

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

impl UnlinkedBerry {
    pub fn link(
        &self,
        firmnesses: &HashMap<BerryFirmnessId, Arc<BerryFirmness>>,
        items: &HashMap<ItemId, Arc<Item>>,
    ) -> Arc<Berry> {
        let berry = Berry {
            id: self.id,
            item: items
                .get(&self.item_id)
                .unwrap_or_else(|| {
                    panic!("No item '{}' found for berry '{}'", self.item_id, self.id)
                })
                .clone(),
            firmness: firmnesses
                .get(&self.firmness_id)
                .unwrap_or_else(|| {
                    panic!(
                        "No firmness '{}' found for berry '{}'",
                        self.firmness_id, self.id
                    )
                })
                .clone(),
            size: self.size,
            max_harvest: self.max_harvest,
            growth_time: self.growth_time,
            soil_dryness: self.soil_dryness,
            smoothness: self.smoothness,
        };
        Arc::new(berry)
    }
}

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
}
