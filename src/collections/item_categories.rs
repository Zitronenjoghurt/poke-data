use crate::data::link_context::LinkContext;
use crate::data_structures::entity_collection::{EntityCollection, HasNameSearchIndex};
use crate::data_structures::string_search_index::StringSearchIndex;
use crate::models::item_category::{ItemCategory, ItemCategoryId};
use crate::traits::has_identifier::IdentifierDictionary;
use crate::traits::has_localized_names::LocalizedNamesDictionary;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ItemCategoriesCollection {
    entities: HashMap<ItemCategoryId, Arc<ItemCategory>>,
    name_search_index: StringSearchIndex<ItemCategoryId>,
}

impl EntityCollection<ItemCategoryId, ItemCategory> for ItemCategoriesCollection {
    fn new(context: &LinkContext) -> Self {
        let entities = context.item_categories.clone();
        let mut dictionary = HashMap::new();
        dictionary.extend(entities.build_identifier_dictionary());
        dictionary.extend(entities.build_localized_name_dictionary());
        let name_search_index = StringSearchIndex::new(dictionary);
        Self {
            entities,
            name_search_index,
        }
    }

    fn entities(&self) -> &HashMap<ItemCategoryId, Arc<ItemCategory>> {
        &self.entities
    }
}

impl HasNameSearchIndex<ItemCategoryId, ItemCategory> for ItemCategoriesCollection {
    fn name_search_index(&self) -> &StringSearchIndex<ItemCategoryId> {
        &self.name_search_index
    }
}
