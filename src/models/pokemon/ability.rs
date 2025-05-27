use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::ability::{Ability, AbilityId};
use crate::models::generation::GenerationId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
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

    fn link(&self, context: &LinkContext) -> Self::Linked {
        PokemonAbility {
            ability: context.abilities.get(&self.ability_id).unwrap().clone(),
            is_hidden: self.is_hidden,
            slot: self.slot,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PokemonAbilityPast {
    pub ability: Option<Arc<Ability>>,
    pub is_hidden: bool,
    pub slot: u8,
}

impl PokemonAbilityPast {
    pub fn to_ability(&self) -> Option<PokemonAbility> {
        self.ability.clone().map(|ability| PokemonAbility {
            ability,
            is_hidden: self.is_hidden,
            slot: self.slot,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedPokemonAbilityPast {
    pub ability_id: Option<AbilityId>,
    pub is_hidden: bool,
    pub slot: u8,
}

impl Linkable for UnlinkedPokemonAbilityPast {
    type Linked = PokemonAbilityPast;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        PokemonAbilityPast {
            ability: self
                .ability_id
                .map(|ability_id| context.abilities.get(&ability_id).unwrap().clone()),
            is_hidden: self.is_hidden,
            slot: self.slot,
        }
    }
}

#[derive(Debug)]
pub struct PokemonAbilitiesPast(HashMap<GenerationId, Vec<PokemonAbilityPast>>);

impl PokemonAbilitiesPast {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get_abilities(&self, generation_id: &GenerationId) -> Option<&Vec<PokemonAbilityPast>> {
        self.0.get(generation_id)
    }

    pub fn get_map(&self) -> &HashMap<GenerationId, Vec<PokemonAbilityPast>> {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedPokemonAbilitiesPast(HashMap<GenerationId, Vec<UnlinkedPokemonAbilityPast>>);

impl UnlinkedPokemonAbilitiesPast {
    pub fn new(abilities: HashMap<GenerationId, Vec<UnlinkedPokemonAbilityPast>>) -> Self {
        Self(abilities)
    }
}

impl Linkable for UnlinkedPokemonAbilitiesPast {
    type Linked = PokemonAbilitiesPast;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let abilities = self
            .0
            .iter()
            .map(|(generation_id, abilities)| {
                (
                    *generation_id,
                    abilities
                        .iter()
                        .map(|ability| ability.link(context))
                        .collect(),
                )
            })
            .collect();
        PokemonAbilitiesPast(abilities)
    }
}
