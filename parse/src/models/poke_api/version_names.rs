use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedName;
use poke_data::models::language::LanguageId;
use poke_data::models::version::VersionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionNameData {
    version_id: VersionId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for VersionNameData {
    fn file_name() -> &'static str {
        "version_names"
    }
}

impl HasId for VersionNameData {
    type Id = VersionId;

    fn id(&self) -> Self::Id {
        self.version_id
    }
}

impl HasLocalizedName for VersionNameData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
