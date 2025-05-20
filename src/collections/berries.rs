use crate::data::link_context::LinkContext;
use crate::data_structures::entity_collection::{EntityCollection, HasNameSearchIndex};
use crate::data_structures::string_search_index::StringSearchIndex;
use crate::models::berry::{Berry, BerryId};
use crate::traits::has_identifier::IdentifierDictionary;
use crate::traits::has_localized_names::LocalizedNamesDictionary;
use std::collections::HashMap;
use std::sync::Arc;

pub struct BerriesCollection {
    entities: HashMap<BerryId, Arc<Berry>>,
    name_search_index: StringSearchIndex<BerryId>,
}

impl EntityCollection<BerryId, Berry> for BerriesCollection {
    fn new(context: &LinkContext) -> Self {
        let entities = context.berries.clone();
        let mut dictionary = HashMap::new();
        dictionary.extend(entities.build_identifier_dictionary());
        dictionary.extend(entities.build_localized_name_dictionary());
        let name_search_index = StringSearchIndex::new(dictionary);
        Self {
            entities,
            name_search_index,
        }
    }

    fn entities(&self) -> &HashMap<BerryId, Arc<Berry>> {
        &self.entities
    }
}

impl HasNameSearchIndex<BerryId, Berry> for BerriesCollection {
    fn name_search_index(&self) -> &StringSearchIndex<BerryId> {
        &self.name_search_index
    }
}
