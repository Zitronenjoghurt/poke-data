use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::ability::{AbilityChangelog, AbilityId};
use poke_data::models::version_group::VersionGroupId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type AbilityChangelogId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityChangelogData {
    id: AbilityChangelogId,
    ability_id: AbilityId,
    changed_in_version_group_id: VersionGroupId,
}

impl PokeApiModel for AbilityChangelogData {
    fn file_name() -> &'static str {
        "ability_changelog"
    }
}

impl HasId for AbilityChangelogData {
    type Id = AbilityChangelogId;

    fn id(&self) -> Self::Id {
        self.ability_id
    }
}

impl IntoModel<AbilityChangelog> for Vec<AbilityChangelogData> {
    fn into_model(self, data: &RawData) -> AbilityChangelog {
        let mut changelog = HashMap::new();
        self.iter().for_each(|entry| {
            let changelog_entries = data.ability_changelog_prose.get_model(&entry.id, data);
            changelog.insert(entry.changed_in_version_group_id, changelog_entries);
        });
        AbilityChangelog::new(changelog)
    }
}
