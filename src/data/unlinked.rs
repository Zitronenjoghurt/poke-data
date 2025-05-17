use crate::data::PokeData;
use crate::models::ability::{AbilityId, UnlinkedAbility};
use crate::models::berry::{BerryId, UnlinkedBerry};
use crate::models::berry_firmness::{BerryFirmness, BerryFirmnessId};
use crate::models::color::{Color, ColorId};
use crate::models::egg_group::{EggGroup, EggGroupId};
use crate::models::evolution_trigger::{EvolutionTrigger, EvolutionTriggerId};
use crate::models::generation::{GenerationId, UnlinkedGeneration};
use crate::models::growth_rate::{GrowthRate, GrowthRateId};
use crate::models::habitat::{Habitat, HabitatId};
use crate::models::item::{ItemId, UnlinkedItem};
use crate::models::pokemon::{PokemonId, UnlinkedPokemon};
use crate::models::region::{Region, RegionId};
use crate::models::shape::{Shape, ShapeId};
use crate::models::species::{SpeciesId, UnlinkedSpecies};
use crate::traits::into_arc_map::IntoArcMap;
use flate2::read::ZlibDecoder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UnlinkedPokeData {
    pub abilities: HashMap<AbilityId, UnlinkedAbility>,
    pub berries: HashMap<BerryId, UnlinkedBerry>,
    pub berry_firmnesses: HashMap<BerryFirmnessId, BerryFirmness>,
    pub colors: HashMap<ColorId, Color>,
    pub egg_groups: HashMap<EggGroupId, EggGroup>,
    pub evolution_triggers: HashMap<EvolutionTriggerId, EvolutionTrigger>,
    pub generations: HashMap<GenerationId, UnlinkedGeneration>,
    pub growth_rates: HashMap<GrowthRateId, GrowthRate>,
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
        let berry_firmnesses = self.berry_firmnesses.clone().into_arc_map();
        let colors = self.colors.clone().into_arc_map();
        let egg_groups = self.egg_groups.clone().into_arc_map();
        let evolution_triggers = self.evolution_triggers.clone().into_arc_map();
        let growth_rates = self.growth_rates.clone().into_arc_map();
        let habitats = self.habitats.clone().into_arc_map();
        let regions = self.regions.clone().into_arc_map();
        let shapes = self.shapes.clone().into_arc_map();

        let items = self
            .items
            .iter()
            .map(|(id, item)| (*id, item.link()))
            .collect();

        let berries = self
            .berries
            .iter()
            .map(|(id, berry)| (*id, berry.link(&berry_firmnesses, &items)))
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
                    species.link(
                        &colors,
                        &generations,
                        &growth_rates,
                        &habitats,
                        &items,
                        &shapes,
                    ),
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
            berries,
            berry_firmnesses,
            colors,
            egg_groups,
            evolution_triggers,
            generations,
            growth_rates,
            habitats,
            items,
            pokemon,
            regions,
            shapes,
            species,
        }
    }
}
