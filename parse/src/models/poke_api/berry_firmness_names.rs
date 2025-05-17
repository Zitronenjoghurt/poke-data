use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedName;
use poke_data::models::berry_firmness::BerryFirmnessId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BerryFirmnessNameData {
    berry_firmness_id: BerryFirmnessId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for BerryFirmnessNameData {
    fn file_name() -> &'static str {
        "berry_firmness_names"
    }
}

impl HasId for BerryFirmnessNameData {
    type Id = BerryFirmnessId;

    fn id(&self) -> Self::Id {
        self.berry_firmness_id
    }
}

impl HasLocalizedName for BerryFirmnessNameData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
