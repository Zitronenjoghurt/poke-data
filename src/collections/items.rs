use crate::data::link_context::LinkContext;
use crate::data_structures::entity_collection::{EntityCollection, HasNameSearchIndex};
use crate::data_structures::string_search_index::StringSearchIndex;
use crate::models::item::{Item, ItemId};
use crate::traits::has_identifier::IdentifierDictionary;
use crate::traits::has_localized_names::LocalizedNamesDictionary;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ItemsCollection {
    entities: HashMap<ItemId, Arc<Item>>,
    name_search_index: StringSearchIndex<ItemId>,
}

impl EntityCollection<ItemId, Item> for ItemsCollection {
    fn new(context: &LinkContext) -> Self {
        let entities = context.items.clone();
        let mut dictionary = HashMap::new();
        dictionary.extend(entities.build_identifier_dictionary());
        dictionary.extend(entities.build_localized_name_dictionary());
        let name_search_index = StringSearchIndex::new(dictionary);
        Self {
            entities,
            name_search_index,
        }
    }

    fn entities(&self) -> &HashMap<ItemId, Arc<Item>> {
        &self.entities
    }
}

impl HasNameSearchIndex<ItemId, Item> for ItemsCollection {
    fn name_search_index(&self) -> &StringSearchIndex<ItemId> {
        &self.name_search_index
    }
}
