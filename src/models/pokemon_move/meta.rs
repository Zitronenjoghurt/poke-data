use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::pokemon_move_ailment::{PokemonMoveAilment, PokemonMoveAilmentId};
use crate::models::pokemon_move_category::{PokemonMoveCategory, PokemonMoveCategoryId};
use crate::types::stat::Stat;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct PokemonMoveMeta {
    pub category: Arc<PokemonMoveCategory>,
    pub ailment: Arc<PokemonMoveAilment>,
    pub min_hits: Option<u8>,
    pub max_hits: Option<u8>,
    pub min_turns: Option<u8>,
    pub max_turns: Option<u8>,
    pub stat_changes: HashMap<Stat, i8>,
    pub drain: i8,
    pub healing: i8,
    pub crit_rate: u8,
    pub ailment_chance: u8,
    pub flinch_chance: u8,
    pub stat_chance: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedPokemonMoveMeta {
    pub category_id: PokemonMoveCategoryId,
    pub ailment_id: PokemonMoveAilmentId,
    pub min_hits: Option<u8>,
    pub max_hits: Option<u8>,
    pub min_turns: Option<u8>,
    pub max_turns: Option<u8>,
    pub stat_changes: HashMap<Stat, i8>,
    pub drain: i8,
    pub healing: i8,
    pub crit_rate: u8,
    pub ailment_chance: u8,
    pub flinch_chance: u8,
    pub stat_chance: u8,
}

impl Linkable for UnlinkedPokemonMoveMeta {
    type Linked = PokemonMoveMeta;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let category = context
            .move_categories
            .get(&self.category_id)
            .unwrap_or_else(|| {
                panic!(
                    "No move category '{}' found for move meta data",
                    self.category_id
                )
            })
            .clone();

        let ailment = context
            .move_ailments
            .get(&self.ailment_id)
            .unwrap_or_else(|| {
                panic!(
                    "No move ailment '{}' found for move meta data",
                    self.ailment_id
                )
            })
            .clone();

        PokemonMoveMeta {
            category,
            ailment,
            min_hits: self.min_hits,
            max_hits: self.max_hits,
            min_turns: self.min_turns,
            max_turns: self.max_turns,
            stat_changes: self.stat_changes.clone(),
            drain: self.drain,
            healing: self.healing,
            crit_rate: self.crit_rate,
            ailment_chance: self.ailment_chance,
            flinch_chance: self.flinch_chance,
            stat_chance: self.stat_chance,
        }
    }
}
