use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedName;
use poke_data::models::language::LanguageId;
use poke_data::models::location_area::LocationAreaId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationAreaProseData {
    location_area_id: LocationAreaId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for LocationAreaProseData {
    fn file_name() -> &'static str {
        "location_area_prose"
    }
}

impl HasId for LocationAreaProseData {
    type Id = LocationAreaId;

    fn id(&self) -> Self::Id {
        self.location_area_id
    }
}

impl HasLocalizedName for LocationAreaProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
