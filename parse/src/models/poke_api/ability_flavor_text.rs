use crate::models::poke_api::PokeApiModel;
use crate::traits::has_flavor_text::HasFlavorText;
use crate::traits::has_id::HasId;
use poke_data::models::ability::AbilityId;
use poke_data::models::language::LanguageId;
use poke_data::models::version_group::VersionGroupId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityFlavorTextData {
    ability_id: AbilityId,
    version_group_id: VersionGroupId,
    language_id: LanguageId,
    flavor_text: String,
}

impl PokeApiModel for AbilityFlavorTextData {
    fn file_name() -> &'static str {
        "ability_flavor_text"
    }
}

impl HasId for AbilityFlavorTextData {
    type Id = AbilityId;

    fn id(&self) -> Self::Id {
        self.ability_id
    }
}

impl HasFlavorText for AbilityFlavorTextData {
    fn language(&self) -> LanguageId {
        self.language_id
    }

    fn version_group(&self) -> VersionGroupId {
        self.version_group_id
    }

    fn text(&self) -> String {
        self.flavor_text.clone()
    }
}
