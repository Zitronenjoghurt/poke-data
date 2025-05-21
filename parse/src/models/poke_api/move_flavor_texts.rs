use crate::models::poke_api::PokeApiModel;
use crate::traits::has_flavor_text::HasFlavorText;
use crate::traits::has_id::HasId;
use poke_data::models::language::LanguageId;
use poke_data::models::pokemon_move::PokemonMoveId;
use poke_data::models::version_group::VersionGroupId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveFlavorTextData {
    move_id: PokemonMoveId,
    version_group_id: VersionGroupId,
    language_id: LanguageId,
    flavor_text: String,
}

impl PokeApiModel for MoveFlavorTextData {
    fn file_name() -> &'static str {
        "move_flavor_text"
    }
}

impl HasId for MoveFlavorTextData {
    type Id = PokemonMoveId;

    fn id(&self) -> Self::Id {
        self.move_id
    }
}

impl HasFlavorText for MoveFlavorTextData {
    fn language(&self) -> LanguageId {
        self.language_id
    }

    fn version_group(&self) -> VersionGroupId {
        self.version_group_id
    }

    fn text(&self) -> String {
        self.flavor_text.clone()
    }
}
