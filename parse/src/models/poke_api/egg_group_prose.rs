use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedName;
use poke_data::models::egg_group::EggGroupId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EggGroupProseData {
    egg_group_id: EggGroupId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for EggGroupProseData {
    fn file_name() -> &'static str {
        "egg_group_prose"
    }
}

impl HasId for EggGroupProseData {
    type Id = EggGroupId;

    fn id(&self) -> Self::Id {
        self.egg_group_id
    }
}

impl HasLocalizedName for EggGroupProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
