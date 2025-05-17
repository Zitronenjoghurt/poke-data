use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedName;
use poke_data::models::habitat::HabitatId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitatNameData {
    pokemon_habitat_id: HabitatId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for HabitatNameData {
    fn file_name() -> &'static str {
        "pokemon_habitat_names"
    }
}

impl HasId for HabitatNameData {
    type Id = HabitatId;

    fn id(&self) -> Self::Id {
        self.pokemon_habitat_id
    }
}

impl HasLocalizedName for HabitatNameData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
