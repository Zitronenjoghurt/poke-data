use crate::data::link_context::LinkContext;
use crate::data_structures::entity_collection::{EntityCollection, HasNameSearchIndex};
use crate::data_structures::string_search_index::StringSearchIndex;
use crate::models::location::{Location, LocationId};
use crate::traits::has_identifier::IdentifierDictionary;
use std::collections::HashMap;
use std::sync::Arc;

pub struct LocationsCollection {
    entities: HashMap<LocationId, Arc<Location>>,
    name_search_index: StringSearchIndex<LocationId>,
}

impl EntityCollection<LocationId, Location> for LocationsCollection {
    fn new(context: &LinkContext) -> Self {
        let entities = context.locations.clone();
        let dictionary = entities.build_identifier_dictionary();
        let name_search_index = StringSearchIndex::new(dictionary);
        Self {
            entities,
            name_search_index,
        }
    }

    fn entities(&self) -> &HashMap<LocationId, Arc<Location>> {
        &self.entities
    }
}

impl HasNameSearchIndex<LocationId, Location> for LocationsCollection {
    fn name_search_index(&self) -> &StringSearchIndex<LocationId> {
        &self.name_search_index
    }
}
