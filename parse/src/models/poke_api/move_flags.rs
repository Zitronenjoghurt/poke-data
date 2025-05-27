use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::pokemon_move_flag::{PokemonMoveFlag, PokemonMoveFlagId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveFlagData {
    id: PokemonMoveFlagId,
    identifier: String,
}

impl PokeApiModel for MoveFlagData {
    fn file_name() -> &'static str {
        "move_flags"
    }
}

impl HasId for MoveFlagData {
    type Id = PokemonMoveFlagId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<PokemonMoveFlag> for MoveFlagData {
    fn into_model(self, data: &RawData) -> PokemonMoveFlag {
        PokemonMoveFlag {
            id: self.id,
            identifier: self.identifier,
            prose: data.move_flag_prose.get_model(&self.id, data),
        }
    }
}
