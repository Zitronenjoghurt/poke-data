use crate::data::linkable::Linkable;
use crate::data::PokeData;
use crate::models::ability::{Ability, AbilityId};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug)]
pub struct PokemonAbility {
    pub ability: Arc<Ability>,
    pub is_hidden: bool,
    pub slot: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedPokemonAbility {
    pub ability_id: AbilityId,
    pub is_hidden: bool,
    pub slot: u8,
}

impl Linkable for UnlinkedPokemonAbility {
    type Linked = PokemonAbility;

    fn link(&self, data: &PokeData) -> Self::Linked {
        PokemonAbility {
            ability: data.abilities.get(&self.ability_id).unwrap().clone(),
            is_hidden: self.is_hidden,
            slot: self.slot,
        }
    }
}
