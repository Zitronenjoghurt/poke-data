use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedString;
use poke_data::models::language::LanguageId;
use poke_data::models::super_contest_effect::SuperContestEffectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperContestEffectProseData {
    super_contest_effect_id: SuperContestEffectId,
    local_language_id: LanguageId,
    flavor_text: String,
}

impl PokeApiModel for SuperContestEffectProseData {
    fn file_name() -> &'static str {
        "super_contest_effect_prose"
    }
}

impl HasId for SuperContestEffectProseData {
    type Id = SuperContestEffectId;

    fn id(&self) -> Self::Id {
        self.super_contest_effect_id
    }
}

impl HasLocalizedString for SuperContestEffectProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn string(&self) -> String {
        self.flavor_text.clone()
    }
}
