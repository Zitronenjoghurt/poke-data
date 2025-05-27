use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::pokemon_move_method::{PokemonMoveMethod, PokemonMoveMethodId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveMethodData {
    id: PokemonMoveMethodId,
    identifier: String,
}

impl PokeApiModel for MoveMethodData {
    fn file_name() -> &'static str {
        "pokemon_move_methods"
    }
}

impl HasId for MoveMethodData {
    type Id = PokemonMoveMethodId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<PokemonMoveMethod> for MoveMethodData {
    fn into_model(self, data: &RawData) -> PokemonMoveMethod {
        PokemonMoveMethod {
            id: self.id,
            identifier: self.identifier,
            prose: data.move_method_prose.get_model(&self.id, data),
        }
    }
}
