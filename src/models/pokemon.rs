use crate::models::ability::{Ability, AbilityId};
use crate::models::pokemon::ability::{PokemonAbility, UnlinkedPokemonAbility};
use crate::models::species::SpeciesId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub mod ability;

pub type PokemonId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedPokemon {
    pub id: PokemonId,
    pub identifier: String,
    pub species_id: SpeciesId,
    pub height: u16,
    pub weight: u16,
    pub base_experience: u16,
    pub order: Option<u16>,
    pub is_default: bool,
    pub abilities: Vec<UnlinkedPokemonAbility>,
}

impl UnlinkedPokemon {
    pub fn link(&self, abilities: &HashMap<AbilityId, Arc<Ability>>) -> Arc<Pokemon> {
        let pokemon = Pokemon {
            id: self.id,
            identifier: self.identifier.clone(),
            species_id: self.species_id,
            height: self.height,
            weight: self.weight,
            base_experience: self.base_experience,
            order: self.order,
            is_default: self.is_default,
            abilities: self.abilities.iter().map(|a| a.link(abilities)).collect(),
        };
        Arc::new(pokemon)
    }
}

#[derive(Debug)]
pub struct Pokemon {
    pub id: PokemonId,
    pub identifier: String,
    pub species_id: SpeciesId,
    pub height: u16,
    pub weight: u16,
    pub base_experience: u16,
    pub order: Option<u16>,
    pub is_default: bool,
    pub abilities: Vec<PokemonAbility>,
}
