use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::flavor_texts::FlavorTexts;
use crate::models::generation::GenerationId;
use crate::models::item_category::{ItemCategory, ItemCategoryId};
use crate::models::item_flag::{ItemFlag, ItemFlagId};
use crate::models::item_fling_effect::{ItemFlingEffect, ItemFlingEffectId};
use crate::models::localized_effects::LocalizedEffects;
use crate::models::localized_names::LocalizedNames;
use crate::traits::has_identifier::HasIdentifier;
use crate::traits::has_localized_names::HasLocalizedNames;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub type ItemId = u16;

#[derive(Debug)]
pub struct Item {
    pub id: ItemId,
    pub identifier: String,
    pub names: LocalizedNames,
    pub flavor_texts: FlavorTexts,
    pub effects: LocalizedEffects,
    pub category: Arc<ItemCategory>,
    pub cost: u32,
    pub fling_power: Option<u8>,
    pub fling_effect: Option<Arc<ItemFlingEffect>>,
    pub flags: Vec<Arc<ItemFlag>>,
    pub game_indices: ItemGameIndices,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemGameIndices(HashMap<GenerationId, u16>);

impl ItemGameIndices {
    pub fn new(indices: HashMap<GenerationId, u16>) -> Self {
        Self(indices)
    }
}

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
    pub flag_ids: Vec<ItemFlagId>,
    pub game_indices: ItemGameIndices,
}

impl Linkable for UnlinkedItem {
    type Linked = Arc<Item>;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let category = context
            .item_categories
            .get(&self.category_id)
            .unwrap_or_else(|| {
                panic!(
                    "No item category '{}' found for item '{}'",
                    self.category_id, self.id
                )
            })
            .clone();

        let fling_effect = self.fling_effect_id.map(|effect_id| {
            context
                .item_fling_effects
                .get(&effect_id)
                .unwrap_or_else(|| {
                    panic!(
                        "No item fling effect '{}' found for item '{}'",
                        effect_id, self.id
                    )
                })
                .clone()
        });

        let flags = self
            .flag_ids
            .iter()
            .map(|flag_id| {
                context
                    .item_flags
                    .get(flag_id)
                    .unwrap_or_else(|| {
                        panic!("No item flag '{}' found for item '{}'", flag_id, self.id)
                    })
                    .clone()
            })
            .collect();

        let item = Item {
            id: self.id,
            identifier: self.identifier.clone(),
            names: self.names.clone(),
            flavor_texts: self.flavor_texts.clone(),
            effects: self.effects.clone(),
            category,
            cost: self.cost,
            fling_power: self.fling_power,
            fling_effect,
            flags,
            game_indices: self.game_indices.clone(),
        };

        Arc::new(item)
    }
}

impl HasLocalizedNames for Item {
    fn localized_names(&self) -> &LocalizedNames {
        &self.names
    }
}

impl HasIdentifier for Item {
    fn identifier(&self) -> &str {
        &self.identifier
    }
}
