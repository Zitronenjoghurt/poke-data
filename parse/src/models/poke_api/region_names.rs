use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedName;
use poke_data::models::language::LanguageId;
use poke_data::models::region::RegionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionNameData {
    region_id: RegionId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for RegionNameData {
    fn file_name() -> &'static str {
        "region_names"
    }
}

impl HasId for RegionNameData {
    type Id = RegionId;

    fn id(&self) -> Self::Id {
        self.region_id
    }
}

impl HasLocalizedName for RegionNameData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
