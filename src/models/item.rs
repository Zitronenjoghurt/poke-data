use crate::models::flavor_texts::FlavorTexts;
use crate::models::item_category::ItemCategoryId;
use crate::models::item_fling_effect::ItemFlingEffectId;
use crate::models::localized_effects::LocalizedEffects;
use crate::models::localized_names::LocalizedNames;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type ItemId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedItem {
    pub id: ItemId,
    pub identifier: String,
    pub names: LocalizedNames,
    pub flavor_texts: FlavorTexts,
    pub effects: LocalizedEffects,
    pub category_id: ItemCategoryId,
    pub cost: u32,
    pub fling_power: Option<u8>,
    pub fling_effect_id: Option<ItemFlingEffectId>,
}

impl UnlinkedItem {
    pub fn link(&self) -> Arc<Item> {
        let item = Item {
            id: self.id,
            identifier: self.identifier.clone(),
            names: self.names.clone(),
            flavor_texts: self.flavor_texts.clone(),
            effects: self.effects.clone(),
            category_id: self.category_id,
            cost: self.cost,
            fling_power: self.fling_power,
            fling_effect_id: self.fling_effect_id,
        };
        Arc::new(item)
    }
}

#[derive(Debug)]
pub struct Item {
    pub id: ItemId,
    pub identifier: String,
    pub names: LocalizedNames,
    pub flavor_texts: FlavorTexts,
    pub effects: LocalizedEffects,
    pub category_id: ItemCategoryId,
    pub cost: u32,
    pub fling_power: Option<u8>,
    pub fling_effect_id: Option<ItemFlingEffectId>,
}
