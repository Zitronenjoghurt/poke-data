use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::ability::AbilityId;
use poke_data::models::generation::GenerationId;
use poke_data::models::pokemon::ability::{
    UnlinkedPokemonAbilitiesPast, UnlinkedPokemonAbilityPast,
};
use poke_data::models::pokemon::PokemonId;
use poke_data::models::version::VersionId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonAbilityPastData {
    pokemon_id: PokemonId,
    generation_id: GenerationId,
    ability_id: Option<AbilityId>,
    is_hidden: u8,
    slot: u8,
}

impl PokeApiModel for PokemonAbilityPastData {
    fn file_name() -> &'static str {
        "pokemon_abilities_past"
    }
}

impl HasId for PokemonAbilityPastData {
    type Id = PokemonId;

    fn id(&self) -> Self::Id {
        self.pokemon_id
    }
}

impl IntoModel<UnlinkedPokemonAbilityPast> for PokemonAbilityPastData {
    fn into_model(self, _data: &RawData) -> UnlinkedPokemonAbilityPast {
        UnlinkedPokemonAbilityPast {
            ability_id: self.ability_id,
            is_hidden: self.is_hidden == 1,
            slot: self.slot,
        }
    }
}

impl IntoModel<UnlinkedPokemonAbilitiesPast> for Vec<PokemonAbilityPastData> {
    fn into_model(self, data: &RawData) -> UnlinkedPokemonAbilitiesPast {
        let abilities = self
            .iter()
            .map(|entry| (entry.generation_id, entry.clone().into_model(data)))
            .fold(
                HashMap::new(),
                |mut acc: HashMap<VersionId, Vec<UnlinkedPokemonAbilityPast>>,
                 (generation_id, ability_past)| {
                    acc.entry(generation_id).or_default().push(ability_past);
                    acc
                },
            );
        UnlinkedPokemonAbilitiesPast::new(abilities)
    }
}
