use crate::data::link_context::LinkContext;
use crate::data_structures::entity_collection::{EntityCollection, HasNameSearchIndex};
use crate::data_structures::string_search_index::StringSearchIndex;
use crate::models::ability::{Ability, AbilityId};
use crate::traits::has_identifier::IdentifierDictionary;
use crate::traits::has_localized_names::LocalizedNamesDictionary;
use std::collections::HashMap;
use std::sync::Arc;

pub struct AbilitiesCollection {
    entities: HashMap<AbilityId, Arc<Ability>>,
    name_search_index: StringSearchIndex<AbilityId>,
}

impl EntityCollection<AbilityId, Ability> for AbilitiesCollection {
    fn new(context: &LinkContext) -> Self {
        let entities = context.abilities.clone();
        let mut dictionary = HashMap::new();
        dictionary.extend(entities.build_identifier_dictionary());
        dictionary.extend(entities.build_localized_name_dictionary());
        let name_search_index = StringSearchIndex::new(dictionary);
        Self {
            entities,
            name_search_index,
        }
    }

    fn entities(&self) -> &HashMap<AbilityId, Arc<Ability>> {
        &self.entities
    }
}

impl HasNameSearchIndex<AbilityId, Ability> for AbilitiesCollection {
    fn name_search_index(&self) -> &StringSearchIndex<AbilityId> {
        &self.name_search_index
    }
}
