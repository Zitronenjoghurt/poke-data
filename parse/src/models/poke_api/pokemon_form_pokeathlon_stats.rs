use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::pokeathlon_stats::PokeathlonStats;
use poke_data::models::pokemon_form::PokemonFormId;
use poke_data::types::pokeathlon_stat::PokeathlonStat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonFormPokeathlonStatData {
    pokemon_form_id: PokemonFormId,
    pokeathlon_stat_id: u8,
    minimum_stat: u8,
    base_stat: u8,
    maximum_stat: u8,
}

impl PokeApiModel for PokemonFormPokeathlonStatData {
    fn file_name() -> &'static str {
        "pokemon_form_pokeathlon_stats"
    }
}

impl HasId for PokemonFormPokeathlonStatData {
    type Id = PokemonFormId;

    fn id(&self) -> Self::Id {
        self.pokemon_form_id
    }
}

impl IntoModel<PokeathlonStats> for Vec<PokemonFormPokeathlonStatData> {
    fn into_model(self, _data: &RawData) -> PokeathlonStats {
        self.iter()
            .fold(PokeathlonStats::default(), |mut stats, entry| {
                let stat = PokeathlonStat::from(entry.pokeathlon_stat_id);
                stats.set_stats(
                    stat,
                    entry.minimum_stat,
                    entry.base_stat,
                    entry.maximum_stat,
                );
                stats
            })
    }
}
