use crate::collections::pokemon_type_efficacies::PokemonTypeEfficaciesCollection;
use crate::data::linkable::Linkable;
use crate::data::PokeData;
use crate::models::ability::{AbilityId, UnlinkedAbility};
use crate::models::berry::{BerryId, UnlinkedBerry};
use crate::models::berry_firmness::{BerryFirmness, BerryFirmnessId};
use crate::models::color::{Color, ColorId};
use crate::models::damage_class::{DamageClass, DamageClassId};
use crate::models::egg_group::{EggGroup, EggGroupId};
use crate::models::encounter_method::{EncounterMethod, EncounterMethodId};
use crate::models::evolution_trigger::{EvolutionTrigger, EvolutionTriggerId};
use crate::models::generation::{GenerationId, UnlinkedGeneration};
use crate::models::growth_rate::{GrowthRate, GrowthRateId};
use crate::models::habitat::{Habitat, HabitatId};
use crate::models::item::{ItemId, UnlinkedItem};
use crate::models::item_category::{ItemCategoryId, UnlinkedItemCategory};
use crate::models::item_flag::{ItemFlag, ItemFlagId};
use crate::models::item_fling_effect::{ItemFlingEffect, ItemFlingEffectId};
use crate::models::item_pocket::{ItemPocket, ItemPocketId};
use crate::models::location::{LocationId, UnlinkedLocation};
use crate::models::location_area::{LocationAreaId, UnlinkedLocationArea};
use crate::models::pokemon::{PokemonId, UnlinkedPokemon};
use crate::models::pokemon_type::{PokemonTypeId, UnlinkedPokemonType};
use crate::models::pokemon_type_efficacies::{
    PokemonTypeEfficacies, PokemonTypeEfficaciesByGeneration,
};
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
    pub damage_classes: HashMap<DamageClassId, DamageClass>,
    pub encounter_methods: HashMap<EncounterMethodId, EncounterMethod>,
    pub egg_groups: HashMap<EggGroupId, EggGroup>,
    pub evolution_triggers: HashMap<EvolutionTriggerId, EvolutionTrigger>,
    pub generations: HashMap<GenerationId, UnlinkedGeneration>,
    pub growth_rates: HashMap<GrowthRateId, GrowthRate>,
    pub habitats: HashMap<HabitatId, Habitat>,
    pub items: HashMap<ItemId, UnlinkedItem>,
    pub item_categories: HashMap<ItemCategoryId, UnlinkedItemCategory>,
    pub item_flags: HashMap<ItemFlagId, ItemFlag>,
    pub item_fling_effects: HashMap<ItemFlingEffectId, ItemFlingEffect>,
    pub item_pockets: HashMap<ItemPocketId, ItemPocket>,
    pub locations: HashMap<LocationId, UnlinkedLocation>,
    pub location_areas: HashMap<LocationAreaId, UnlinkedLocationArea>,
    pub pokemon: HashMap<PokemonId, UnlinkedPokemon>,
    pub pokemon_types: HashMap<PokemonTypeId, UnlinkedPokemonType>,
    pub pokemon_type_efficacies: PokemonTypeEfficacies,
    pub pokemon_type_efficacies_past: PokemonTypeEfficaciesByGeneration,
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
        let mut data = PokeData::default();
        data.berry_firmnesses = self.berry_firmnesses.clone().into_arc_map();
        data.colors = self.colors.clone().into_arc_map();
        data.damage_classes = self.damage_classes.clone().into_arc_map();
        data.encounter_methods = self.encounter_methods.clone().into_arc_map();
        data.egg_groups = self.egg_groups.clone().into_arc_map();
        data.evolution_triggers = self.evolution_triggers.clone().into_arc_map();
        data.growth_rates = self.growth_rates.clone().into_arc_map();
        data.habitats = self.habitats.clone().into_arc_map();
        data.item_flags = self.item_flags.clone().into_arc_map();
        data.item_fling_effects = self.item_fling_effects.clone().into_arc_map();
        data.item_pockets = self.item_pockets.clone().into_arc_map();
        data.regions = self.regions.clone().into_arc_map();
        data.shapes = self.shapes.clone().into_arc_map();

        data.locations = self.locations.link(&data);
        data.location_areas = self.location_areas.link(&data);

        data.item_categories = self.item_categories.link(&data);
        data.items = self.items.link(&data);

        data.berries = self.berries.link(&data);

        data.generations = self.generations.link(&data);

        data.version_groups = self.version_groups.link(&data);
        data.versions = self.versions.link(&data);

        data.abilities = self.abilities.link(&data);
        data.pokemon_types = self.pokemon_types.link(&data);
        data.pokemon_type_efficacies = PokemonTypeEfficaciesCollection::build_from_unlinked(self);
        data.species = self.species.link(&data);

        data.pokemon = self.pokemon.link(&data);

        data
    }
}
