use crate::models::poke_api::ability_changelog::AbilityChangelogId;
use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::ability::AbilityChangelogEntry;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityChangelogProseData {
    ability_changelog_id: AbilityChangelogId,
    local_language_id: LanguageId,
    effect: String,
}

impl PokeApiModel for AbilityChangelogProseData {
    fn file_name() -> &'static str {
        "ability_changelog_prose"
    }
}

impl HasId for AbilityChangelogProseData {
    type Id = AbilityChangelogId;

    fn id(&self) -> Self::Id {
        self.ability_changelog_id
    }
}

impl IntoModel<AbilityChangelogEntry> for AbilityChangelogProseData {
    fn into_model(self, _data: &RawData) -> AbilityChangelogEntry {
        AbilityChangelogEntry {
            language: self.local_language_id,
            effect: self.effect.clone(),
        }
    }
}
