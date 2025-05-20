use crate::data::link_context::LinkContext;
use crate::data_structures::entity_collection::{EntityCollection, HasNameSearchIndex};
use crate::data_structures::string_search_index::StringSearchIndex;
use crate::models::location_area::{LocationArea, LocationAreaId};
use crate::traits::has_localized_names::LocalizedNamesDictionary;
use std::collections::HashMap;
use std::sync::Arc;

pub struct LocationAreasCollection {
    entities: HashMap<LocationAreaId, Arc<LocationArea>>,
    name_search_index: StringSearchIndex<LocationAreaId>,
}

impl EntityCollection<LocationAreaId, LocationArea> for LocationAreasCollection {
    fn new(context: &LinkContext) -> Self {
        let entities = context.location_areas.clone();
        let dictionary = entities.build_localized_name_dictionary();
        let name_search_index = StringSearchIndex::new(dictionary);
        Self {
            entities,
            name_search_index,
        }
    }

    fn entities(&self) -> &HashMap<LocationAreaId, Arc<LocationArea>> {
        &self.entities
    }
}

impl HasNameSearchIndex<LocationAreaId, LocationArea> for LocationAreasCollection {
    fn name_search_index(&self) -> &StringSearchIndex<LocationAreaId> {
        &self.name_search_index
    }
}
