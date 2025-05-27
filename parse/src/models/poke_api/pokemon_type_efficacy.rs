use crate::models::poke_api::PokeApiModel;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::pokemon_type::PokemonTypeId;
use poke_data::models::pokemon_type_efficacies::PokemonTypeEfficacies;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonTypeEfficacyData {
    damage_type_id: PokemonTypeId,
    target_type_id: PokemonTypeId,
    damage_factor: u8,
}

impl PokeApiModel for PokemonTypeEfficacyData {
    fn file_name() -> &'static str {
        "type_efficacy"
    }
}

impl IntoModel<PokemonTypeEfficacies> for Vec<PokemonTypeEfficacyData> {
    fn into_model(self, _data: &RawData) -> PokemonTypeEfficacies {
        let efficacies = self
            .into_iter()
            .map(|efficacy| {
                (
                    (efficacy.damage_type_id, efficacy.target_type_id),
                    efficacy.damage_factor,
                )
            })
            .collect();
        PokemonTypeEfficacies::new(efficacies)
    }
}
