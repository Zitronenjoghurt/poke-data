use crate::data::PokeData;
use crate::models::ability::{AbilityId, UnlinkedAbility};
use crate::models::berry::{BerryId, UnlinkedBerry};
use crate::models::berry_firmness::{BerryFirmness, BerryFirmnessId};
use crate::models::color::{Color, ColorId};
use crate::models::egg_group::{EggGroup, EggGroupId};
use crate::models::encounter_method::{EncounterMethod, EncounterMethodId};
use crate::models::evolution_trigger::{EvolutionTrigger, EvolutionTriggerId};
use crate::models::generation::{GenerationId, UnlinkedGeneration};
use crate::models::growth_rate::{GrowthRate, GrowthRateId};
use crate::models::habitat::{Habitat, HabitatId};
use crate::models::item::{ItemId, UnlinkedItem};
use crate::models::location::{LocationId, UnlinkedLocation};
use crate::models::location_area::{LocationAreaId, UnlinkedLocationArea};
use crate::models::pokemon::{PokemonId, UnlinkedPokemon};
use crate::models::region::{Region, RegionId};
use crate::models::shape::{Shape, ShapeId};
use crate::models::species::{SpeciesId, UnlinkedSpecies};
use crate::models::version::{UnlinkedVersion, VersionId};
use crate::models::version_group::{UnlinkedVersionGroup, VersionGroupId};
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
    pub encounter_methods: HashMap<EncounterMethodId, EncounterMethod>,
    pub egg_groups: HashMap<EggGroupId, EggGroup>,
    pub evolution_triggers: HashMap<EvolutionTriggerId, EvolutionTrigger>,
    pub generations: HashMap<GenerationId, UnlinkedGeneration>,
    pub growth_rates: HashMap<GrowthRateId, GrowthRate>,
    pub habitats: HashMap<HabitatId, Habitat>,
    pub items: HashMap<ItemId, UnlinkedItem>,
    pub locations: HashMap<LocationId, UnlinkedLocation>,
    pub location_areas: HashMap<LocationAreaId, UnlinkedLocationArea>,
    pub pokemon: HashMap<PokemonId, UnlinkedPokemon>,
    pub regions: HashMap<RegionId, Region>,
    pub shapes: HashMap<ShapeId, Shape>,
    pub species: HashMap<SpeciesId, UnlinkedSpecies>,
    pub versions: HashMap<VersionId, UnlinkedVersion>,
    pub version_groups: HashMap<VersionGroupId, UnlinkedVersionGroup>,
}

impl UnlinkedPokeData {
    pub fn load(compressed_data_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let data = std::fs::read(compressed_data_path)?;
        let mut decompressor = ZlibDecoder::new(data.as_slice());
        let mut decompressed_data = Vec::new();
        decompressor.read_to_end(&mut decompressed_data)?;
        Ok(bincode::deserialize(&decompressed_data)?)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(bincode::deserialize(data)?)
    }

    pub fn initialize(&self) -> PokeData {
        let berry_firmnesses = self.berry_firmnesses.clone().into_arc_map();
        let colors = self.colors.clone().into_arc_map();
        let encounter_methods = self.encounter_methods.clone().into_arc_map();
        let egg_groups = self.egg_groups.clone().into_arc_map();
        let evolution_triggers = self.evolution_triggers.clone().into_arc_map();
        let growth_rates = self.growth_rates.clone().into_arc_map();
        let habitats = self.habitats.clone().into_arc_map();
        let regions = self.regions.clone().into_arc_map();
        let shapes = self.shapes.clone().into_arc_map();

        let locations = self
            .locations
            .iter()
            .map(|(id, location)| (*id, location.link(&regions)))
            .collect();

        let location_areas = self
            .location_areas
            .iter()
            .map(|(id, location_area)| (*id, location_area.link(&locations)))
            .collect();

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

        let version_groups = self
            .version_groups
            .iter()
            .map(|(id, version_group)| (*id, version_group.link(&generations, &regions)))
            .collect();

        let versions = self
            .versions
            .iter()
            .map(|(id, version)| (*id, version.link(&version_groups)))
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
            encounter_methods,
            egg_groups,
            evolution_triggers,
            generations,
            growth_rates,
            habitats,
            items,
            locations,
            location_areas,
            pokemon,
            regions,
            shapes,
            species,
            version_groups,
            versions,
        }
    }
}
