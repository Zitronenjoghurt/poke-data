use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedString;
use poke_data::models::ability::AbilityId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityNameData {
    ability_id: AbilityId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for AbilityNameData {
    fn file_name() -> &'static str {
        "ability_names"
    }
}

impl HasId for AbilityNameData {
    type Id = AbilityId;

    fn id(&self) -> Self::Id {
        self.ability_id
    }
}

impl HasLocalizedString for AbilityNameData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn string(&self) -> String {
        self.name.clone()
    }
}
