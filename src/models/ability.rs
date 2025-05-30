use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::flavor_texts::FlavorTexts;
use crate::models::generation::{Generation, GenerationId};
use crate::models::language::LanguageId;
use crate::models::localized_effects::LocalizedEffects;
use crate::models::localized_names::LocalizedStrings;
use crate::models::version_group::VersionGroupId;
use crate::traits::has_identifier::HasIdentifier;
use crate::traits::has_localized_names::HasLocalizedNames;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub type AbilityId = u16;

#[derive(Debug)]
pub struct Ability {
    pub id: AbilityId,
    pub identifier: String,
    pub names: LocalizedStrings,
    pub flavor_texts: FlavorTexts,
    pub effects: LocalizedEffects,
    pub changelog: AbilityChangelog,
    pub generation: Arc<Generation>,
    pub is_main_series: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityChangelog(HashMap<VersionGroupId, Vec<AbilityChangelogEntry>>);

impl AbilityChangelog {
    pub fn new(changelogs: HashMap<VersionGroupId, Vec<AbilityChangelogEntry>>) -> Self {
        Self(changelogs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityChangelogEntry {
    pub language: LanguageId,
    pub effect: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedAbility {
    pub id: AbilityId,
    pub identifier: String,
    pub names: LocalizedStrings,
    pub flavor_texts: FlavorTexts,
    pub effects: LocalizedEffects,
    pub changelog: AbilityChangelog,
    pub generation_id: GenerationId,
    pub is_main_series: bool,
}

impl Linkable for UnlinkedAbility {
    type Linked = Arc<Ability>;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let generation = context
            .generations
            .get(&self.generation_id)
            .unwrap_or_else(|| {
                panic!(
                    "No generation '{}' found for ability '{}'",
                    self.generation_id, self.id
                )
            })
            .clone();

        let ability = Ability {
            id: self.id,
            identifier: self.identifier.clone(),
            names: self.names.clone(),
            flavor_texts: self.flavor_texts.clone(),
            effects: self.effects.clone(),
            changelog: self.changelog.clone(),
            generation,
            is_main_series: self.is_main_series,
        };

        Arc::new(ability)
    }
}

impl HasIdentifier for Ability {
    fn identifier(&self) -> &str {
        &self.identifier
    }
}

impl HasLocalizedNames for Ability {
    fn localized_names(&self) -> &LocalizedStrings {
        &self.names
    }
}
