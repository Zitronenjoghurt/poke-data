use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::damage_class::DamageClassId;
use poke_data::models::generation::GenerationId;
use poke_data::models::pokemon_type::{PokemonTypeId, UnlinkedPokemonType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonTypeData {
    id: PokemonTypeId,
    identifier: String,
    generation_id: GenerationId,
    damage_class_id: Option<DamageClassId>,
}

impl PokeApiModel for PokemonTypeData {
    fn file_name() -> &'static str {
        "types"
    }
}

impl HasId for PokemonTypeData {
    type Id = PokemonTypeId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<UnlinkedPokemonType> for PokemonTypeData {
    fn into_model(self, data: &RawData) -> UnlinkedPokemonType {
        UnlinkedPokemonType {
            id: self.id,
            identifier: self.identifier,
            names: data.pokemon_type_names.get_model(&self.id, data),
            generation_id: self.generation_id,
            damage_class_id: self.damage_class_id,
            game_indices: data.pokemon_type_game_indices.get_model(&self.id, data),
        }
    }
}
