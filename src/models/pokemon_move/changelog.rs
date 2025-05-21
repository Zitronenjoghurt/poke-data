use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::pokemon_move_effect::{PokemonMoveEffect, PokemonMoveEffectId};
use crate::models::pokemon_move_target::{PokemonMoveTarget, PokemonMoveTargetId};
use crate::types::pokemon_type::Type;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug)]
pub struct PokemonMoveChangelog {
    pub pokemon_type: Option<Type>,
    pub power: Option<u8>,
    pub pp: Option<u8>,
    pub accuracy: Option<u8>,
    pub priority: Option<i8>,
    pub target: Option<Arc<PokemonMoveTarget>>,
    pub effect: Option<Arc<PokemonMoveEffect>>,
    pub effect_chance: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedPokemonMoveChangelog {
    pub pokemon_type: Option<Type>,
    pub power: Option<u8>,
    pub pp: Option<u8>,
    pub accuracy: Option<u8>,
    pub priority: Option<i8>,
    pub target_id: Option<PokemonMoveTargetId>,
    pub effect_id: Option<PokemonMoveEffectId>,
    pub effect_chance: Option<u8>,
}

impl Linkable for UnlinkedPokemonMoveChangelog {
    type Linked = PokemonMoveChangelog;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let target = self.target_id.map(|target_id| {
            context
                .move_targets
                .get(&target_id)
                .unwrap_or_else(|| {
                    panic!(
                        "No move target '{}' found for move changelog entry",
                        target_id
                    )
                })
                .clone()
        });

        let effect = self.effect_id.map(|effect_id| {
            context
                .move_effects
                .get(&effect_id)
                .unwrap_or_else(|| {
                    panic!(
                        "No move effect '{}' found for move changelog entry",
                        effect_id
                    )
                })
                .clone()
        });

        PokemonMoveChangelog {
            pokemon_type: self.pokemon_type,
            power: self.power,
            pp: self.pp,
            accuracy: self.accuracy,
            priority: self.priority,
            target,
            effect,
            effect_chance: self.effect_chance,
        }
    }
}
