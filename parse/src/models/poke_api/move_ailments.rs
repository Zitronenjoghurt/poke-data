use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::pokemon_move_ailment::{PokemonMoveAilment, PokemonMoveAilmentId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveAilmentData {
    id: PokemonMoveAilmentId,
    identifier: String,
}

impl PokeApiModel for MoveAilmentData {
    fn file_name() -> &'static str {
        "move_meta_ailments"
    }
}

impl HasId for MoveAilmentData {
    type Id = PokemonMoveAilmentId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<PokemonMoveAilment> for MoveAilmentData {
    fn into_model(self, data: &RawData) -> PokemonMoveAilment {
        PokemonMoveAilment {
            id: self.id,
            identifier: self.identifier,
            names: data.move_ailment_names.get_model(&self.id, data),
        }
    }
}
