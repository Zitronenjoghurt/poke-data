use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::item::{Item, ItemId};
use crate::models::version::VersionId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct PokemonWildItems(HashMap<VersionId, Vec<PokemonWildItem>>);

#[derive(Debug)]
pub struct PokemonWildItem {
    pub item: Arc<Item>,
    pub rarity: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedPokemonWildItems(HashMap<VersionId, Vec<UnlinkedPokemonWildItem>>);

impl UnlinkedPokemonWildItems {
    pub fn new(items: HashMap<VersionId, Vec<UnlinkedPokemonWildItem>>) -> Self {
        Self(items)
    }
}

impl Linkable for UnlinkedPokemonWildItems {
    type Linked = PokemonWildItems;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let items = self
            .0
            .iter()
            .map(|(version_id, items)| {
                (
                    *version_id,
                    items.iter().map(|item| item.link(context)).collect(),
                )
            })
            .collect();
        PokemonWildItems(items)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedPokemonWildItem {
    pub item_id: ItemId,
    pub rarity: u8,
}

impl Linkable for UnlinkedPokemonWildItem {
    type Linked = PokemonWildItem;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let item = context
            .items
            .get(&self.item_id)
            .unwrap_or_else(|| {
                panic!(
                    "No item '{}' found for pokemon wild held item",
                    self.item_id
                )
            })
            .clone();

        PokemonWildItem {
            item,
            rarity: self.rarity,
        }
    }
}
