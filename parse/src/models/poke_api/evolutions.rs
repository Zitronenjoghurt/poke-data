use crate::models::poke_api::PokeApiModel;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::evolution_trigger::EvolutionTriggerId;
use poke_data::models::item::ItemId;
use poke_data::models::location::LocationId;
use poke_data::models::pokemon_move::PokemonMoveId;
use poke_data::models::species::evolution::UnlinkedEvolution;
use poke_data::models::species::SpeciesId;
use poke_data::types::gender::Gender;
use poke_data::types::pokemon_type::Type;
use poke_data::types::time_of_day::TimeOfDay;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionData {
    pub evolved_species_id: SpeciesId,
    evolution_trigger_id: EvolutionTriggerId,
    trigger_item_id: Option<ItemId>,
    minimum_level: Option<u8>,
    gender_id: Option<u8>,
    location_id: Option<LocationId>,
    held_item_id: Option<ItemId>,
    time_of_day: Option<String>,
    known_move_id: Option<PokemonMoveId>,
    known_move_type_id: Option<u16>,
    minimum_happiness: Option<u8>,
    minimum_beauty: Option<u8>,
    minimum_affection: Option<u8>,
    relative_physical_stats: Option<i8>,
    party_species_id: Option<SpeciesId>,
    party_type_id: Option<u16>,
    trade_species_id: Option<SpeciesId>,
    needs_overworld_rain: Option<u8>,
    turn_upside_down: Option<u8>,
}

impl PokeApiModel for EvolutionData {
    fn file_name() -> &'static str {
        "pokemon_evolution"
    }
}

impl IntoModel<UnlinkedEvolution> for EvolutionData {
    fn into_model(self, _data: &RawData) -> UnlinkedEvolution {
        UnlinkedEvolution {
            evolves_into_species_id: self.evolved_species_id,
            trigger_id: self.evolution_trigger_id,
            trigger_item_id: self.trigger_item_id,
            minimum_level: self.minimum_level,
            gender: self.gender_id.map(Gender::from),
            location_id: self.location_id,
            held_item_id: self.held_item_id,
            time_of_day: self
                .time_of_day
                .map(|time_of_day| TimeOfDay::from_str(&time_of_day).unwrap()),
            known_move_id: self.known_move_id,
            known_move_type: self.known_move_type_id.map(Type::from),
            min_happiness: self.minimum_happiness,
            min_beauty: self.minimum_beauty,
            min_affection: self.minimum_affection,
            relative_physical_stats: self.relative_physical_stats,
            party_species_id: self.party_species_id,
            party_type: self.party_type_id.map(Type::from),
            trade_species_id: self.trade_species_id,
            needs_overworld_rain: self.needs_overworld_rain.map(|needs_rain| needs_rain == 1),
            turn_upside_down: self.turn_upside_down.map(|upside_down| upside_down == 1),
        }
    }
}
