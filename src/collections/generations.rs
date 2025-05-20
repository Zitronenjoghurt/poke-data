use crate::data::link_context::LinkContext;
use crate::data_structures::entity_collection::{EntityCollection, HasNameSearchIndex};
use crate::data_structures::string_search_index::StringSearchIndex;
use crate::models::generation::{Generation, GenerationId};
use crate::traits::has_identifier::IdentifierDictionary;
use crate::traits::has_localized_names::LocalizedNamesDictionary;
use std::collections::HashMap;
use std::sync::Arc;

pub struct GenerationsCollection {
    entities: HashMap<GenerationId, Arc<Generation>>,
    name_search_index: StringSearchIndex<GenerationId>,
}

impl EntityCollection<GenerationId, Generation> for GenerationsCollection {
    fn new(context: &LinkContext) -> Self {
        let entities = context.generations.clone();
        let mut dictionary = HashMap::new();
        dictionary.extend(entities.build_identifier_dictionary());
        dictionary.extend(entities.build_localized_name_dictionary());
        let name_search_index = StringSearchIndex::new(dictionary);
        Self {
            entities,
            name_search_index,
        }
    }

    fn entities(&self) -> &HashMap<GenerationId, Arc<Generation>> {
        &self.entities
    }
}

impl HasNameSearchIndex<GenerationId, Generation> for GenerationsCollection {
    fn name_search_index(&self) -> &StringSearchIndex<GenerationId> {
        &self.name_search_index
    }
}
