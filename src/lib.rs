use crate::models::ability::{Ability, AbilityId, UnlinkedAbility};
use crate::models::egg_group::{EggGroup, EggGroupId};
use crate::models::generation::{Generation, GenerationId, UnlinkedGeneration};
use crate::models::pokemon::{Pokemon, PokemonId, UnlinkedPokemon};
use crate::models::region::{Region, RegionId};
use crate::traits::into_arc_map::IntoArcMap;
use flate2::read::ZlibDecoder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

pub mod models;
#[cfg(test)]
mod tests;
mod traits;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UnlinkedPokeData {
    pub abilities: HashMap<AbilityId, UnlinkedAbility>,
    pub egg_groups: HashMap<EggGroupId, EggGroup>,
    pub generations: HashMap<GenerationId, UnlinkedGeneration>,
    pub pokemon: HashMap<PokemonId, UnlinkedPokemon>,
    pub regions: HashMap<RegionId, Region>,
}

impl UnlinkedPokeData {
    pub fn load(compressed_data_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let data = std::fs::read(compressed_data_path)?;
        let mut decompressor = ZlibDecoder::new(data.as_slice());
        let mut decompressed_data = Vec::new();
        decompressor.read_to_end(&mut decompressed_data)?;
        Ok(bincode::deserialize(&decompressed_data)?)
    }

    pub fn initialize(&self) -> PokeData {
        let egg_groups = self.egg_groups.clone().into_arc_map();
        let regions = self.regions.clone().into_arc_map();

        let generations = self
            .generations
            .iter()
            .map(|(id, generation)| (*id, generation.link(&regions)))
            .collect();

        let abilities = self
            .abilities
            .iter()
            .map(|(id, ability)| (*id, ability.link(&generations)))
            .collect();

        let pokemon = self
            .pokemon
            .iter()
            .map(|(id, pokemon)| (*id, pokemon.link(&abilities)))
            .collect();

        PokeData {
            abilities,
            egg_groups,
            generations,
            pokemon,
            regions,
        }
    }
}

#[derive(Debug)]
pub struct PokeData {
    pub abilities: HashMap<AbilityId, Arc<Ability>>,
    pub egg_groups: HashMap<EggGroupId, Arc<EggGroup>>,
    pub generations: HashMap<GenerationId, Arc<Generation>>,
    pub pokemon: HashMap<PokemonId, Arc<Pokemon>>,
    pub regions: HashMap<RegionId, Arc<Region>>,
}

impl PokeData {
    pub fn load(compressed_data_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let unlinked_data = UnlinkedPokeData::load(compressed_data_path)?;
        Ok(unlinked_data.initialize())
    }
}
