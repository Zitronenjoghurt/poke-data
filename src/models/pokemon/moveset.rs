use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::pokemon_move::{PokemonMove, PokemonMoveId};
use crate::models::pokemon_move_method::{PokemonMoveMethod, PokemonMoveMethodId};
use crate::models::version_group::VersionGroupId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct Moveset(HashMap<VersionGroupId, Vec<MovesetEntry>>);

impl Moveset {
    pub fn get_moves(&self, version_group_id: VersionGroupId) -> Option<&Vec<MovesetEntry>> {
        self.0.get(&version_group_id)
    }
}

#[derive(Debug)]
pub struct MovesetEntry {
    pub pokemon_move: Arc<PokemonMove>,
    pub move_method: Arc<PokemonMoveMethod>,
    pub level: u8,
    pub order: Option<u8>,
    pub mastery: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedMoveset(HashMap<VersionGroupId, Vec<UnlinkedMovesetEntry>>);

impl UnlinkedMoveset {
    pub fn new(entries: HashMap<VersionGroupId, Vec<UnlinkedMovesetEntry>>) -> Self {
        Self(entries)
    }
}

impl Linkable for UnlinkedMoveset {
    type Linked = Moveset;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let entries = self
            .0
            .iter()
            .map(|(version_group_id, entry)| (*version_group_id, entry.link(context)))
            .collect();
        Moveset(entries)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedMovesetEntry {
    pub move_id: PokemonMoveId,
    pub move_method_id: PokemonMoveMethodId,
    pub level: u8,
    pub order: Option<u8>,
    pub mastery: Option<u8>,
}

impl Linkable for UnlinkedMovesetEntry {
    type Linked = MovesetEntry;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let pokemon_move = context
            .moves
            .get(&self.move_id)
            .unwrap_or_else(|| panic!("No move '{}' found for pokemon moveset", self.move_id))
            .clone();

        let move_method = context
            .move_methods
            .get(&self.move_method_id)
            .unwrap_or_else(|| {
                panic!(
                    "No move method '{}' found for pokemon moveset",
                    self.move_method_id
                )
            })
            .clone();

        MovesetEntry {
            pokemon_move,
            move_method,
            level: self.level,
            order: self.order,
            mastery: self.mastery,
        }
    }
}
