use crate::data::link_context::LinkContext;
use crate::data_structures::entity_collection::{EntityCollection, HasNameSearchIndex};
use crate::data_structures::string_search_index::StringSearchIndex;
use crate::models::region::{Region, RegionId};
use crate::traits::has_identifier::IdentifierDictionary;
use crate::traits::has_localized_names::LocalizedNamesDictionary;
use std::collections::HashMap;
use std::sync::Arc;

pub struct RegionsCollection {
    entities: HashMap<RegionId, Arc<Region>>,
    name_search_index: StringSearchIndex<RegionId>,
}

impl EntityCollection<RegionId, Region> for RegionsCollection {
    fn new(context: &LinkContext) -> Self {
        let entities = context.regions.clone();
        let mut dictionary = HashMap::new();
        dictionary.extend(entities.build_identifier_dictionary());
        dictionary.extend(entities.build_localized_name_dictionary());
        let name_search_index = StringSearchIndex::new(dictionary);
        Self {
            entities,
            name_search_index,
        }
    }

    fn entities(&self) -> &HashMap<RegionId, Arc<Region>> {
        &self.entities
    }
}

impl HasNameSearchIndex<RegionId, Region> for RegionsCollection {
    fn name_search_index(&self) -> &StringSearchIndex<RegionId> {
        &self.name_search_index
    }
}
