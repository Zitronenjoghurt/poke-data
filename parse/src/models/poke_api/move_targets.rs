use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::pokemon_move_target::{PokemonMoveTarget, PokemonMoveTargetId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveTargetData {
    id: PokemonMoveTargetId,
    identifier: String,
}

impl PokeApiModel for MoveTargetData {
    fn file_name() -> &'static str {
        "move_targets"
    }
}

impl HasId for MoveTargetData {
    type Id = PokemonMoveTargetId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<PokemonMoveTarget> for MoveTargetData {
    fn into_model(self, data: &RawData) -> PokemonMoveTarget {
        PokemonMoveTarget {
            id: self.id,
            identifier: self.identifier,
            prose: data.move_target_prose.get_model(&self.id, data),
        }
    }
}
