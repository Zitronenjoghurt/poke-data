use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedName;
use poke_data::models::generation::GenerationId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationNameData {
    generation_id: GenerationId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for GenerationNameData {
    fn file_name() -> &'static str {
        "generation_names"
    }
}

impl HasId for GenerationNameData {
    type Id = GenerationId;

    fn id(&self) -> Self::Id {
        self.generation_id
    }
}

impl HasLocalizedName for GenerationNameData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
