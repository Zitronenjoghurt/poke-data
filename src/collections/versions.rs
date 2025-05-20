use crate::data::link_context::LinkContext;
use crate::data_structures::entity_collection::{EntityCollection, HasNameSearchIndex};
use crate::data_structures::string_search_index::StringSearchIndex;
use crate::models::version::{Version, VersionId};
use crate::traits::has_identifier::IdentifierDictionary;
use crate::traits::has_localized_names::LocalizedNamesDictionary;
use std::collections::HashMap;
use std::sync::Arc;

pub struct VersionsCollection {
    entities: HashMap<VersionId, Arc<Version>>,
    name_search_index: StringSearchIndex<VersionId>,
}

impl EntityCollection<VersionId, Version> for VersionsCollection {
    fn new(context: &LinkContext) -> Self {
        let entities = context.versions.clone();
        let mut dictionary = HashMap::new();
        dictionary.extend(entities.build_identifier_dictionary());
        dictionary.extend(entities.build_localized_name_dictionary());
        let name_search_index = StringSearchIndex::new(dictionary);
        Self {
            entities,
            name_search_index,
        }
    }

    fn entities(&self) -> &HashMap<VersionId, Arc<Version>> {
        &self.entities
    }
}

impl HasNameSearchIndex<VersionId, Version> for VersionsCollection {
    fn name_search_index(&self) -> &StringSearchIndex<VersionId> {
        &self.name_search_index
    }
}
