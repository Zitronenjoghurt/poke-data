use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::base_stats::BaseStats;
use poke_data::models::pokemon::PokemonId;
use poke_data::types::stat::Stat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonStatData {
    pokemon_id: PokemonId,
    stat_id: u8,
    base_stat: u8,
    effort: u8,
}

impl PokeApiModel for PokemonStatData {
    fn file_name() -> &'static str {
        "pokemon_stats"
    }
}

impl HasId for PokemonStatData {
    type Id = PokemonId;

    fn id(&self) -> Self::Id {
        self.pokemon_id
    }
}

impl IntoModel<BaseStats> for Vec<PokemonStatData> {
    fn into_model(self, _data: &RawData) -> BaseStats {
        let mut base_stats = BaseStats::default();
        self.iter().for_each(|stat| {
            base_stats.set_stat(Stat::from(stat.stat_id), stat.base_stat, stat.effort)
        });
        base_stats
    }
}
