use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedName;
use poke_data::models::growth_rate::GrowthRateId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthRateProseData {
    growth_rate_id: GrowthRateId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for GrowthRateProseData {
    fn file_name() -> &'static str {
        "growth_rate_prose"
    }
}

impl HasId for GrowthRateProseData {
    type Id = GrowthRateId;

    fn id(&self) -> Self::Id {
        self.growth_rate_id
    }
}

impl HasLocalizedName for GrowthRateProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
