use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedName;
use poke_data::models::language::LanguageId;
use poke_data::models::pokemon_type::PokemonTypeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonTypeNameData {
    type_id: PokemonTypeId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for PokemonTypeNameData {
    fn file_name() -> &'static str {
        "type_names"
    }
}

impl HasId for PokemonTypeNameData {
    type Id = PokemonTypeId;

    fn id(&self) -> Self::Id {
        self.type_id
    }
}

impl HasLocalizedName for PokemonTypeNameData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
