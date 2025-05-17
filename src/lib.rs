use crate::models::ability::{Ability, AbilityId, UnlinkedAbility};
use crate::models::color::{Color, ColorId};
use crate::models::egg_group::{EggGroup, EggGroupId};
use crate::models::generation::{Generation, GenerationId, UnlinkedGeneration};
use crate::models::habitat::{Habitat, HabitatId};
use crate::models::item::{Item, ItemId, UnlinkedItem};
use crate::models::pokemon::{Pokemon, PokemonId, UnlinkedPokemon};
use crate::models::region::{Region, RegionId};
use crate::models::shape::{Shape, ShapeId};
use crate::models::species::{Species, SpeciesId, UnlinkedSpecies};
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
pub mod types;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UnlinkedPokeData {
    pub abilities: HashMap<AbilityId, UnlinkedAbility>,
    pub colors: HashMap<ColorId, Color>,
    pub egg_groups: HashMap<EggGroupId, EggGroup>,
    pub generations: HashMap<GenerationId, UnlinkedGeneration>,
    pub habitats: HashMap<HabitatId, Habitat>,
    pub items: HashMap<ItemId, UnlinkedItem>,
    pub pokemon: HashMap<PokemonId, UnlinkedPokemon>,
    pub regions: HashMap<RegionId, Region>,
    pub shapes: HashMap<ShapeId, Shape>,
    pub species: HashMap<SpeciesId, UnlinkedSpecies>,
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
        let colors = self.colors.clone().into_arc_map();
        let egg_groups = self.egg_groups.clone().into_arc_map();
        let habitats = self.habitats.clone().into_arc_map();
        let regions = self.regions.clone().into_arc_map();
        let shapes = self.shapes.clone().into_arc_map();

        let items = self
            .items
            .iter()
            .map(|(id, item)| (*id, item.link()))
            .collect();

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

        let species = self
            .species
            .iter()
            .map(|(id, species)| {
                (
                    *id,
                    species.link(&colors, &generations, &habitats, &items, &shapes),
                )
            })
            .collect();

        let pokemon = self
            .pokemon
            .iter()
            .map(|(id, pokemon)| (*id, pokemon.link(&abilities, &species)))
            .collect();

        PokeData {
            abilities,
            colors,
            egg_groups,
            generations,
            habitats,
            items,
            pokemon,
            regions,
            shapes,
            species,
        }
    }
}

#[derive(Debug)]
pub struct PokeData {
    pub abilities: HashMap<AbilityId, Arc<Ability>>,
    pub colors: HashMap<ColorId, Arc<Color>>,
    pub egg_groups: HashMap<EggGroupId, Arc<EggGroup>>,
    pub generations: HashMap<GenerationId, Arc<Generation>>,
    pub habitats: HashMap<HabitatId, Arc<Habitat>>,
    pub items: HashMap<ItemId, Arc<Item>>,
    pub pokemon: HashMap<PokemonId, Arc<Pokemon>>,
    pub regions: HashMap<RegionId, Arc<Region>>,
    pub shapes: HashMap<ShapeId, Arc<Shape>>,
    pub species: HashMap<SpeciesId, Arc<Species>>,
}

impl PokeData {
    pub fn load(compressed_data_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let unlinked_data = UnlinkedPokeData::load(compressed_data_path)?;
        Ok(unlinked_data.initialize())
    }
}
