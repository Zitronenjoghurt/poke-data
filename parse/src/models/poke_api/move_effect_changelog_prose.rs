use crate::models::poke_api::move_effect_changelog::MoveEffectChangelogId;
use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_effects::HasLocalizedEffects;
use crate::traits::has_localized_name::HasLocalizedString;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveEffectChangelogProseData {
    move_effect_changelog_id: MoveEffectChangelogId,
    local_language_id: LanguageId,
    effect: String,
}

impl PokeApiModel for MoveEffectChangelogProseData {
    fn file_name() -> &'static str {
        "move_effect_changelog_prose"
    }
}

impl HasId for MoveEffectChangelogProseData {
    type Id = MoveEffectChangelogId;

    fn id(&self) -> Self::Id {
        self.move_effect_changelog_id
    }
}

impl HasLocalizedString for MoveEffectChangelogProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn string(&self) -> String {
        self.effect.clone()
    }
}
