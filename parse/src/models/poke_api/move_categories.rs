use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::pokemon_move_category::{PokemonMoveCategory, PokemonMoveCategoryId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveCategoryData {
    id: PokemonMoveCategoryId,
    identifier: String,
}

impl PokeApiModel for MoveCategoryData {
    fn file_name() -> &'static str {
        "move_meta_categories"
    }
}

impl HasId for MoveCategoryData {
    type Id = PokemonMoveCategoryId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<PokemonMoveCategory> for MoveCategoryData {
    fn into_model(self, data: &RawData) -> PokemonMoveCategory {
        PokemonMoveCategory {
            id: self.id,
            identifier: self.identifier,
            description: data.move_category_prose.get_model(&self.id, data),
        }
    }
}
