use crate::data::link_context::LinkContext;
use crate::data_structures::entity_collection::{EntityCollection, HasNameSearchIndex};
use crate::data_structures::string_search_index::StringSearchIndex;
use crate::models::pokemon_move::{PokemonMove, PokemonMoveId};
use crate::traits::has_identifier::IdentifierDictionary;
use crate::traits::has_localized_names::LocalizedNamesDictionary;
use std::collections::HashMap;
use std::sync::Arc;

pub struct MovesCollection {
    entities: HashMap<PokemonMoveId, Arc<PokemonMove>>,
    name_search_index: StringSearchIndex<PokemonMoveId>,
}

impl EntityCollection<PokemonMoveId, PokemonMove> for MovesCollection {
    fn new(context: &LinkContext) -> Self {
        let entities = context.moves.clone();
        let mut dictionary = HashMap::new();
        dictionary.extend(entities.build_identifier_dictionary());
        dictionary.extend(entities.build_localized_name_dictionary());
        let name_search_index = StringSearchIndex::new(dictionary);
        Self {
            entities,
            name_search_index,
        }
    }

    fn entities(&self) -> &HashMap<PokemonMoveId, Arc<PokemonMove>> {
        &self.entities
    }
}

impl HasNameSearchIndex<PokemonMoveId, PokemonMove> for MovesCollection {
    fn name_search_index(&self) -> &StringSearchIndex<PokemonMoveId> {
        &self.name_search_index
    }
}
