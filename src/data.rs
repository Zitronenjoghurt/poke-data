use crate::collections::pokemon_type_efficacies::PokemonTypeEfficaciesCollection;
use crate::models::ability::{Ability, AbilityId};
use crate::models::berry::{Berry, BerryId};
use crate::models::berry_firmness::{BerryFirmness, BerryFirmnessId};
use crate::models::color::{Color, ColorId};
use crate::models::damage_class::{DamageClass, DamageClassId};
use crate::models::egg_group::{EggGroup, EggGroupId};
use crate::models::encounter::{Encounter, EncounterId};
use crate::models::encounter_condition::{EncounterCondition, EncounterConditionId};
use crate::models::encounter_condition_value::{
    EncounterConditionValue, EncounterConditionValueId,
};
use crate::models::encounter_method::{EncounterMethod, EncounterMethodId};
use crate::models::evolution_trigger::{EvolutionTrigger, EvolutionTriggerId};
use crate::models::generation::{Generation, GenerationId};
use crate::models::growth_rate::{GrowthRate, GrowthRateId};
use crate::models::habitat::{Habitat, HabitatId};
use crate::models::item::{Item, ItemId};
use crate::models::item_category::{ItemCategory, ItemCategoryId};
use crate::models::item_flag::{ItemFlag, ItemFlagId};
use crate::models::item_fling_effect::{ItemFlingEffect, ItemFlingEffectId};
use crate::models::item_pocket::{ItemPocket, ItemPocketId};
use crate::models::location::{Location, LocationId};
use crate::models::location_area::{LocationArea, LocationAreaId};
use crate::models::pokemon::{Pokemon, PokemonId};
use crate::models::pokemon_type::{PokemonType, PokemonTypeId};
use crate::models::region::{Region, RegionId};
use crate::models::shape::{Shape, ShapeId};
use crate::models::species::{Species, SpeciesId};
use crate::models::version::{Version, VersionId};
use crate::models::version_group::{VersionGroup, VersionGroupId};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use unlinked::UnlinkedPokeData;

pub mod linkable;
pub mod unlinked;

#[derive(Debug, Default)]
pub struct PokeData {
    pub abilities: HashMap<AbilityId, Arc<Ability>>,
    pub berries: HashMap<BerryId, Arc<Berry>>,
    pub berry_firmnesses: HashMap<BerryFirmnessId, Arc<BerryFirmness>>,
    pub colors: HashMap<ColorId, Arc<Color>>,
    pub damage_classes: HashMap<DamageClassId, Arc<DamageClass>>,
    pub encounters: HashMap<EncounterId, Arc<Encounter>>,
    pub encounter_conditions: HashMap<EncounterConditionId, Arc<EncounterCondition>>,
    pub encounter_condition_values:
        HashMap<EncounterConditionValueId, Arc<EncounterConditionValue>>,
    pub encounter_methods: HashMap<EncounterMethodId, Arc<EncounterMethod>>,
    pub egg_groups: HashMap<EggGroupId, Arc<EggGroup>>,
    pub evolution_triggers: HashMap<EvolutionTriggerId, Arc<EvolutionTrigger>>,
    pub generations: HashMap<GenerationId, Arc<Generation>>,
    pub growth_rates: HashMap<GrowthRateId, Arc<GrowthRate>>,
    pub habitats: HashMap<HabitatId, Arc<Habitat>>,
    pub items: HashMap<ItemId, Arc<Item>>,
    pub item_categories: HashMap<ItemCategoryId, Arc<ItemCategory>>,
    pub item_flags: HashMap<ItemFlagId, Arc<ItemFlag>>,
    pub item_fling_effects: HashMap<ItemFlingEffectId, Arc<ItemFlingEffect>>,
    pub item_pockets: HashMap<ItemPocketId, Arc<ItemPocket>>,
    pub locations: HashMap<LocationId, Arc<Location>>,
    pub location_areas: HashMap<LocationAreaId, Arc<LocationArea>>,
    pub pokemon: HashMap<PokemonId, Arc<Pokemon>>,
    pub pokemon_types: HashMap<PokemonTypeId, Arc<PokemonType>>,
    pub pokemon_type_efficacies: PokemonTypeEfficaciesCollection,
    pub regions: HashMap<RegionId, Arc<Region>>,
    pub shapes: HashMap<ShapeId, Arc<Shape>>,
    pub species: HashMap<SpeciesId, Arc<Species>>,
    pub versions: HashMap<VersionId, Arc<Version>>,
    pub version_groups: HashMap<VersionGroupId, Arc<VersionGroup>>,
}

impl PokeData {
    pub fn load_path(compressed_data_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let unlinked_data = UnlinkedPokeData::load(compressed_data_path)?;
        Ok(unlinked_data.initialize())
    }

    pub fn load_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let unlinked_data = UnlinkedPokeData::from_bytes(bytes)?;
        Ok(unlinked_data.initialize())
    }
}
