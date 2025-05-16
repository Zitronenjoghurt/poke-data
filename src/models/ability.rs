use crate::models::flavor_texts::FlavorTexts;
use crate::models::generation::GenerationId;
use crate::models::language::LanguageId;
use crate::models::localized_effects::LocalizedEffects;
use crate::models::localized_names::LocalizedNames;
use crate::models::version_group::VersionGroupId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type AbilityId = u16;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ability {
    pub id: AbilityId,
    pub identifier: String,
    pub names: LocalizedNames,
    pub flavor_texts: FlavorTexts,
    pub effects: LocalizedEffects,
    pub changelog: AbilityChangelog,
    pub generation_id: GenerationId,
    pub is_main_series: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbilityChangelog(HashMap<VersionGroupId, Vec<AbilityChangelogEntry>>);

impl AbilityChangelog {
    pub fn new(changelogs: HashMap<VersionGroupId, Vec<AbilityChangelogEntry>>) -> Self {
        Self(changelogs)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbilityChangelogEntry {
    pub language: LanguageId,
    pub effect: String,
}
