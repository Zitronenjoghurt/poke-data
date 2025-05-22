use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name_description::HasLocalizedNameDescription;
use poke_data::models::language::LanguageId;
use poke_data::models::pokedex::PokedexId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokedexProseData {
    pokedex_id: PokedexId,
    local_language_id: LanguageId,
    name: String,
    description: String,
}

impl PokeApiModel for PokedexProseData {
    fn file_name() -> &'static str {
        "pokedex_prose"
    }
}

impl HasId for PokedexProseData {
    type Id = PokedexId;

    fn id(&self) -> Self::Id {
        self.pokedex_id
    }
}

impl HasLocalizedNameDescription for PokedexProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}
