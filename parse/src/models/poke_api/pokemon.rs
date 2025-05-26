use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::pokemon::{PokemonId, UnlinkedPokemon};
use poke_data::models::species::SpeciesId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonData {
    id: PokemonId,
    identifier: String,
    species_id: SpeciesId,
    height: u16,
    weight: u16,
    base_experience: u16,
    order: Option<u16>,
    is_default: u8,
}

impl PokeApiModel for PokemonData {
    fn file_name() -> &'static str {
        "pokemon"
    }
}

impl HasId for PokemonData {
    type Id = PokemonId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<UnlinkedPokemon> for PokemonData {
    fn into_model(self, data: &RawData) -> UnlinkedPokemon {
        UnlinkedPokemon {
            id: self.id,
            identifier: self.identifier,
            species_id: self.species_id,
            types: data.pokemon_types_map.get_model(&self.id, data),
            past_types: data.pokemon_types_past_map.get_model(&self.id, data),
            height: self.height,
            weight: self.weight,
            base_stats: data.pokemon_stats.get_model(&self.id, data),
            base_experience: self.base_experience,
            order: self.order,
            is_default: self.is_default == 1,
            abilities: data.pokemon_abilities.get_model(&self.id, data),
            moveset: data.pokemon_move_map.get_model(&self.id, data),
            form_ids: data
                .pokemon_form_id_map
                .get(&self.id)
                .cloned()
                .unwrap_or_default(),
        }
    }
}
