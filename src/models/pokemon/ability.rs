use crate::models::ability::{Ability, AbilityId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedPokemonAbility {
    pub ability_id: AbilityId,
    pub is_hidden: bool,
    pub slot: u8,
}

impl UnlinkedPokemonAbility {
    pub fn link(&self, abilities: &HashMap<AbilityId, Arc<Ability>>) -> PokemonAbility {
        PokemonAbility {
            ability: abilities.get(&self.ability_id).unwrap().clone(),
            is_hidden: self.is_hidden,
            slot: self.slot,
        }
    }
}

#[derive(Debug)]
pub struct PokemonAbility {
    pub ability: Arc<Ability>,
    pub is_hidden: bool,
    pub slot: u8,
}
