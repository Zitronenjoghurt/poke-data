use crate::data::link_context::LinkContext;
use crate::data_structures::entity_collection::{EntityCollection, HasNameSearchIndex};
use crate::data_structures::string_search_index::StringSearchIndex;
use crate::models::species::{Species, SpeciesId};
use crate::traits::has_identifier::IdentifierDictionary;
use crate::traits::has_localized_names::LocalizedNamesDictionary;
use std::collections::HashMap;
use std::sync::Arc;

pub struct SpeciesCollection {
    entities: HashMap<SpeciesId, Arc<Species>>,
    name_search_index: StringSearchIndex<SpeciesId>,
}

impl EntityCollection<SpeciesId, Species> for SpeciesCollection {
    fn new(context: &LinkContext) -> Self {
        let entities = context.species.clone();
        let mut dictionary = HashMap::new();
        dictionary.extend(entities.build_identifier_dictionary());
        dictionary.extend(entities.build_localized_name_dictionary());
        let name_search_index = StringSearchIndex::new(dictionary);
        Self {
            entities,
            name_search_index,
        }
    }

    fn entities(&self) -> &HashMap<SpeciesId, Arc<Species>> {
        &self.entities
    }
}

impl HasNameSearchIndex<SpeciesId, Species> for SpeciesCollection {
    fn name_search_index(&self) -> &StringSearchIndex<SpeciesId> {
        &self.name_search_index
    }
}
