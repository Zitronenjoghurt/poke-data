use crate::data::link_context::LinkContext;
use crate::data_structures::entity_collection::{EntityCollection, HasNameSearchIndex};
use crate::data_structures::string_search_index::StringSearchIndex;
use crate::models::pokemon::{Pokemon, PokemonId};
use crate::traits::has_identifier::IdentifierDictionary;
use std::collections::HashMap;
use std::sync::Arc;

pub struct PokemonCollection {
    entities: HashMap<PokemonId, Arc<Pokemon>>,
    name_search_index: StringSearchIndex<PokemonId>,
}

impl EntityCollection<PokemonId, Pokemon> for PokemonCollection {
    fn new(context: &LinkContext) -> Self {
        let entities = context.pokemon.clone();
        let dictionary = entities.build_identifier_dictionary();
        let name_search_index = StringSearchIndex::new(dictionary);
        Self {
            entities,
            name_search_index,
        }
    }

    fn entities(&self) -> &HashMap<PokemonId, Arc<Pokemon>> {
        &self.entities
    }
}

impl HasNameSearchIndex<PokemonId, Pokemon> for PokemonCollection {
    fn name_search_index(&self) -> &StringSearchIndex<PokemonId> {
        &self.name_search_index
    }
}
