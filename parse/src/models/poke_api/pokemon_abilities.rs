use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::ability::AbilityId;
use poke_data::models::pokemon::ability::UnlinkedPokemonAbility;
use poke_data::models::pokemon::PokemonId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonAbilityData {
    pokemon_id: PokemonId,
    ability_id: AbilityId,
    is_hidden: u8,
    slot: u8,
}

impl PokeApiModel for PokemonAbilityData {
    fn file_name() -> &'static str {
        "pokemon_abilities"
    }
}

impl HasId for PokemonAbilityData {
    type Id = PokemonId;

    fn id(&self) -> Self::Id {
        self.pokemon_id
    }
}

impl IntoModel<UnlinkedPokemonAbility> for PokemonAbilityData {
    fn into_model(self, _data: &RawData) -> UnlinkedPokemonAbility {
        UnlinkedPokemonAbility {
            ability_id: self.ability_id,
            is_hidden: self.is_hidden == 1,
            slot: self.slot,
        }
    }
}
