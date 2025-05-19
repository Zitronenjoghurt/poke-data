use crate::models::poke_api::PokeApiModel;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::generation::GenerationId;
use poke_data::models::pokemon_type::PokemonTypeId;
use poke_data::models::pokemon_type_efficacies::{
    PokemonTypeEfficacies, PokemonTypeEfficaciesByGeneration,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonTypeEfficacyPastData {
    damage_type_id: PokemonTypeId,
    target_type_id: PokemonTypeId,
    damage_factor: u8,
    generation_id: GenerationId,
}

impl PokeApiModel for PokemonTypeEfficacyPastData {
    fn file_name() -> &'static str {
        "type_efficacy_past"
    }
}

impl IntoModel<PokemonTypeEfficaciesByGeneration> for Vec<PokemonTypeEfficacyPastData> {
    fn into_model(self, _data: &RawData) -> PokemonTypeEfficaciesByGeneration {
        let mut efficacies_by_gen: HashMap<
            GenerationId,
            HashMap<(PokemonTypeId, PokemonTypeId), u8>,
        > = HashMap::new();

        for efficacy in self {
            let gen_map = efficacies_by_gen.entry(efficacy.generation_id).or_default();

            gen_map.insert(
                (efficacy.damage_type_id, efficacy.target_type_id),
                efficacy.damage_factor,
            );
        }

        let efficacies = efficacies_by_gen
            .into_iter()
            .map(|(gen_id, efficacies_map)| (gen_id, PokemonTypeEfficacies::new(efficacies_map)))
            .collect();

        PokemonTypeEfficaciesByGeneration::new(efficacies)
    }
}
