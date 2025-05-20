use crate::data::link_context::LinkContext;
use crate::data_structures::entity_collection::{EntityCollection, HasNameSearchIndex};
use crate::data_structures::string_search_index::StringSearchIndex;
use crate::models::egg_group::{EggGroup, EggGroupId};
use crate::traits::has_identifier::IdentifierDictionary;
use crate::traits::has_localized_names::LocalizedNamesDictionary;
use std::collections::HashMap;
use std::sync::Arc;

pub struct EggGroupsCollection {
    entities: HashMap<EggGroupId, Arc<EggGroup>>,
    name_search_index: StringSearchIndex<EggGroupId>,
}

impl EntityCollection<EggGroupId, EggGroup> for EggGroupsCollection {
    fn new(context: &LinkContext) -> Self {
        let entities = context.egg_groups.clone();
        let mut dictionary = HashMap::new();
        dictionary.extend(entities.build_identifier_dictionary());
        dictionary.extend(entities.build_localized_name_dictionary());
        let name_search_index = StringSearchIndex::new(dictionary);
        Self {
            entities,
            name_search_index,
        }
    }

    fn entities(&self) -> &HashMap<EggGroupId, Arc<EggGroup>> {
        &self.entities
    }
}

impl HasNameSearchIndex<EggGroupId, EggGroup> for EggGroupsCollection {
    fn name_search_index(&self) -> &StringSearchIndex<EggGroupId> {
        &self.name_search_index
    }
}
