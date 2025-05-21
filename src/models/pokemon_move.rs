use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::contest_effect::{ContestEffect, ContestEffectId};
use crate::models::contest_type::{ContestType, ContestTypeId};
use crate::models::damage_class::{DamageClass, DamageClassId};
use crate::models::flavor_texts::FlavorTexts;
use crate::models::generation::{Generation, GenerationId};
use crate::models::localized_names::LocalizedStrings;
use crate::models::pokemon_move::changelog::{PokemonMoveChangelog, UnlinkedPokemonMoveChangelog};
use crate::models::pokemon_move::meta::{PokemonMoveMeta, UnlinkedPokemonMoveMeta};
use crate::models::pokemon_move_effect::{PokemonMoveEffect, PokemonMoveEffectId};
use crate::models::pokemon_move_flag::{PokemonMoveFlag, PokemonMoveFlagId};
use crate::models::pokemon_move_target::{PokemonMoveTarget, PokemonMoveTargetId};
use crate::models::super_contest_effect::{SuperContestEffect, SuperContestEffectId};
use crate::models::version_group::VersionGroupId;
use crate::traits::has_identifier::HasIdentifier;
use crate::traits::has_localized_names::HasLocalizedNames;
use crate::types::pokemon_type::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub mod changelog;
pub mod meta;

pub type PokemonMoveId = u16;

#[derive(Debug)]
pub struct PokemonMove {
    pub id: PokemonMoveId,
    pub identifier: String,
    pub names: LocalizedStrings,
    pub flavor_texts: FlavorTexts,
    pub generation: Arc<Generation>,
    pub pokemon_type: Type,
    pub power: Option<u8>,
    pub pp: Option<u8>,
    pub accuracy: Option<u8>,
    pub priority: i8,
    pub target: Arc<PokemonMoveTarget>,
    pub damage_class: Arc<DamageClass>,
    pub effect: Option<Arc<PokemonMoveEffect>>,
    pub effect_chance: Option<u8>,
    pub contest_type: Option<Arc<ContestType>>,
    pub contest_effect: Option<Arc<ContestEffect>>,
    pub super_contest_effect: Option<Arc<SuperContestEffect>>,
    pub changelogs: HashMap<VersionGroupId, PokemonMoveChangelog>,
    pub flags: Vec<Arc<PokemonMoveFlag>>,
    pub meta_data: Option<PokemonMoveMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedPokemonMove {
    pub id: PokemonMoveId,
    pub identifier: String,
    pub names: LocalizedStrings,
    pub flavor_texts: FlavorTexts,
    pub generation_id: GenerationId,
    pub pokemon_type: Type,
    pub power: Option<u8>,
    pub pp: Option<u8>,
    pub accuracy: Option<u8>,
    pub priority: i8,
    pub target_id: PokemonMoveTargetId,
    pub damage_class_id: DamageClassId,
    pub effect_id: Option<PokemonMoveEffectId>,
    pub effect_chance: Option<u8>,
    pub contest_type_id: Option<ContestTypeId>,
    pub contest_effect_id: Option<ContestEffectId>,
    pub super_contest_effect_id: Option<SuperContestEffectId>,
    pub changelogs: HashMap<VersionGroupId, UnlinkedPokemonMoveChangelog>,
    pub flag_ids: Vec<PokemonMoveFlagId>,
    pub meta_data: Option<UnlinkedPokemonMoveMeta>,
}

impl Linkable for UnlinkedPokemonMove {
    type Linked = Arc<PokemonMove>;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let generation = context
            .generations
            .get(&self.generation_id)
            .unwrap_or_else(|| {
                panic!(
                    "No generation '{}' found for move '{}'",
                    self.generation_id, self.id
                )
            })
            .clone();

        let target = context
            .move_targets
            .get(&self.target_id)
            .unwrap_or_else(|| {
                panic!(
                    "No move target '{}' found for move '{}'",
                    self.target_id, self.id
                )
            })
            .clone();

        let damage_class = context
            .damage_classes
            .get(&self.damage_class_id)
            .unwrap_or_else(|| {
                panic!(
                    "No move damage class '{}' found for move '{}'",
                    self.damage_class_id, self.id
                )
            })
            .clone();

        let effect = self.effect_id.map(|effect_id| {
            context
                .move_effects
                .get(&effect_id)
                .unwrap_or_else(|| {
                    panic!(
                        "No move effect '{}' found for move '{}'",
                        effect_id, self.id
                    )
                })
                .clone()
        });

        let contest_type = self.contest_type_id.map(|contest_type_id| {
            context
                .contest_types
                .get(&contest_type_id)
                .unwrap_or_else(|| {
                    panic!(
                        "No contest type '{}' found for move '{}'",
                        contest_type_id, self.id
                    )
                })
                .clone()
        });

        let contest_effect = self.contest_effect_id.map(|contest_effect_id| {
            context
                .contest_effects
                .get(&contest_effect_id)
                .unwrap_or_else(|| {
                    panic!(
                        "No contest effect '{}' found for move '{}'",
                        contest_effect_id, self.id
                    )
                })
                .clone()
        });

        let super_contest_effect = self.super_contest_effect_id.map(|super_contest_effect_id| {
            context
                .super_contest_effects
                .get(&super_contest_effect_id)
                .unwrap_or_else(|| {
                    panic!(
                        "No super contest effect '{}' found for move '{}'",
                        super_contest_effect_id, self.id
                    )
                })
                .clone()
        });

        let changelogs = self
            .changelogs
            .iter()
            .map(|(version_group_id, changelog)| (*version_group_id, changelog.link(context)))
            .collect();

        let flags = self
            .flag_ids
            .iter()
            .map(|flag_id| {
                context
                    .move_flags
                    .get(flag_id)
                    .unwrap_or_else(|| {
                        panic!("No move flag '{}' found for move '{}'", flag_id, self.id)
                    })
                    .clone()
            })
            .collect();

        let meta_data = self.meta_data.clone().map(|data| data.link(context));

        let pokemon_move = PokemonMove {
            id: self.id,
            identifier: self.identifier.clone(),
            names: self.names.clone(),
            flavor_texts: self.flavor_texts.clone(),
            generation,
            pokemon_type: self.pokemon_type,
            power: self.power,
            pp: self.pp,
            accuracy: self.accuracy,
            priority: self.priority,
            target,
            damage_class,
            effect,
            effect_chance: self.effect_chance,
            contest_type,
            contest_effect,
            super_contest_effect,
            changelogs,
            flags,
            meta_data,
        };

        Arc::new(pokemon_move)
    }
}

impl HasIdentifier for PokemonMove {
    fn identifier(&self) -> &str {
        &self.identifier
    }
}

impl HasLocalizedNames for PokemonMove {
    fn localized_names(&self) -> &LocalizedStrings {
        &self.names
    }
}
