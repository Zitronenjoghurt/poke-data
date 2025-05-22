use crate::data::link_context::LinkContext;
use crate::data_structures::entity_collection::{EntityCollection, HasNameSearchIndex};
use crate::data_structures::string_search_index::StringSearchIndex;
use crate::models::pokedex::{Pokedex, PokedexId};
use crate::traits::has_identifier::IdentifierDictionary;
use crate::traits::has_localized_name_descriptions::LocalizedNameDescriptionDictionary;
use std::collections::HashMap;
use std::sync::Arc;

pub struct PokedexCollection {
    entities: HashMap<PokedexId, Arc<Pokedex>>,
    name_search_index: StringSearchIndex<PokedexId>,
}

impl EntityCollection<PokedexId, Pokedex> for PokedexCollection {
    fn new(context: &LinkContext) -> Self {
        let entities = context.pokedexes.clone();
        let mut dictionary = HashMap::new();
        dictionary.extend(entities.build_identifier_dictionary());
        dictionary.extend(entities.build_localized_name_dictionary());
        let name_search_index = StringSearchIndex::new(dictionary);
        Self {
            entities,
            name_search_index,
        }
    }

    fn entities(&self) -> &HashMap<PokedexId, Arc<Pokedex>> {
        &self.entities
    }
}

impl HasNameSearchIndex<PokedexId, Pokedex> for PokedexCollection {
    fn name_search_index(&self) -> &StringSearchIndex<PokedexId> {
        &self.name_search_index
    }
}
