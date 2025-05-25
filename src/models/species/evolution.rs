use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::evolution_trigger::{EvolutionTrigger, EvolutionTriggerId};
use crate::models::item::{Item, ItemId};
use crate::models::location::{Location, LocationId};
use crate::models::pokemon_move::{PokemonMove, PokemonMoveId};
use crate::models::species::SpeciesId;
use crate::types::gender::Gender;
use crate::types::pokemon_type::Type;
use crate::types::time_of_day::TimeOfDay;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug)]
pub struct Evolution {
    pub evolves_into_species_id: SpeciesId,
    pub trigger: Arc<EvolutionTrigger>,
    pub trigger_item: Option<Arc<Item>>,
    pub minimum_level: Option<u8>,
    pub gender: Option<Gender>,
    pub location: Option<Arc<Location>>,
    pub held_item: Option<Arc<Item>>,
    pub time_of_day: Option<TimeOfDay>,
    pub known_move: Option<Arc<PokemonMove>>,
    pub known_move_type: Option<Type>,
    pub min_happiness: Option<u8>,
    pub min_beauty: Option<u8>,
    pub min_affection: Option<u8>,
    pub relative_physical_stats: Option<i8>,
    pub party_species_id: Option<SpeciesId>,
    pub party_type: Option<Type>,
    pub trade_species_id: Option<SpeciesId>,
    pub needs_overworld_rain: Option<bool>,
    pub turn_upside_down: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedEvolution {
    pub evolves_into_species_id: SpeciesId,
    pub trigger_id: EvolutionTriggerId,
    pub trigger_item_id: Option<ItemId>,
    pub minimum_level: Option<u8>,
    pub gender: Option<Gender>,
    pub location_id: Option<LocationId>,
    pub held_item_id: Option<ItemId>,
    pub time_of_day: Option<TimeOfDay>,
    pub known_move_id: Option<PokemonMoveId>,
    pub known_move_type: Option<Type>,
    pub min_happiness: Option<u8>,
    pub min_beauty: Option<u8>,
    pub min_affection: Option<u8>,
    pub relative_physical_stats: Option<i8>,
    pub party_species_id: Option<SpeciesId>,
    pub party_type: Option<Type>,
    pub trade_species_id: Option<SpeciesId>,
    pub needs_overworld_rain: Option<bool>,
    pub turn_upside_down: Option<bool>,
}

impl Linkable for UnlinkedEvolution {
    type Linked = Evolution;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let trigger = context
            .evolution_triggers
            .get(&self.trigger_id)
            .unwrap_or_else(|| {
                panic!(
                    "No evolution trigger '{}' found for evolution",
                    self.trigger_id
                )
            })
            .clone();

        let trigger_item = self.trigger_item_id.map(|item_id| {
            context
                .items
                .get(&item_id)
                .unwrap_or_else(|| panic!("No trigger item '{}' found for evolution", item_id))
                .clone()
        });

        let location = self.location_id.map(|location_id| {
            context
                .locations
                .get(&location_id)
                .unwrap_or_else(|| panic!("No location '{}' found for evolution", location_id))
                .clone()
        });

        let held_item = self.held_item_id.map(|item_id| {
            context
                .items
                .get(&item_id)
                .unwrap_or_else(|| panic!("No held item '{}' found for evolution", item_id))
                .clone()
        });

        let known_move = self.known_move_id.map(|move_id| {
            context
                .moves
                .get(&move_id)
                .unwrap_or_else(|| panic!("No known move '{}' found for evolution", move_id))
                .clone()
        });

        Evolution {
            evolves_into_species_id: self.evolves_into_species_id,
            trigger,
            trigger_item,
            minimum_level: self.minimum_level,
            gender: self.gender,
            location,
            held_item,
            time_of_day: self.time_of_day,
            known_move,
            known_move_type: self.known_move_type,
            min_happiness: self.min_happiness,
            min_beauty: self.min_beauty,
            min_affection: self.min_affection,
            relative_physical_stats: self.relative_physical_stats,
            party_species_id: self.party_species_id,
            party_type: self.party_type,
            trade_species_id: self.trade_species_id,
            needs_overworld_rain: self.needs_overworld_rain,
            turn_upside_down: self.turn_upside_down,
        }
    }
}
