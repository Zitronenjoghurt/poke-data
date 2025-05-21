use crate::collections::abilities::AbilitiesCollection;
use crate::collections::berries::BerriesCollection;
use crate::collections::egg_groups::EggGroupsCollection;
use crate::collections::generations::GenerationsCollection;
use crate::collections::item_categories::ItemCategoriesCollection;
use crate::collections::items::ItemsCollection;
use crate::collections::location_areas::LocationAreasCollection;
use crate::collections::locations::LocationsCollection;
use crate::collections::pokemon::PokemonCollection;
use crate::collections::pokemon_type_efficacies::PokemonTypeEfficaciesCollection;
use crate::collections::regions::RegionsCollection;
use crate::collections::species::SpeciesCollection;
use crate::collections::versions::VersionsCollection;
use crate::data::PokeData;
use crate::data_structures::entity_collection::EntityCollection;
use crate::models::ability::{Ability, AbilityId};
use crate::models::berry::{Berry, BerryId};
use crate::models::berry_firmness::{BerryFirmness, BerryFirmnessId};
use crate::models::berry_flavor::{BerryFlavor, BerryFlavorId};
use crate::models::color::{Color, ColorId};
use crate::models::contest_effect::{ContestEffect, ContestEffectId};
use crate::models::contest_type::{ContestType, ContestTypeId};
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
use crate::models::pokemon_move_target::{PokemonMoveTarget, PokemonMoveTargetId};
use crate::models::pokemon_type::{PokemonType, PokemonTypeId};
use crate::models::region::{Region, RegionId};
use crate::models::shape::{Shape, ShapeId};
use crate::models::species::{Species, SpeciesId};
use crate::models::super_contest_effect::{SuperContestEffect, SuperContestEffectId};
use crate::models::version::{Version, VersionId};
use crate::models::version_group::{VersionGroup, VersionGroupId};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub struct LinkContext {
    pub abilities: HashMap<AbilityId, Arc<Ability>>,
    pub berries: HashMap<BerryId, Arc<Berry>>,
    pub berry_firmnesses: HashMap<BerryFirmnessId, Arc<BerryFirmness>>,
    pub berry_flavors: HashMap<BerryFlavorId, Arc<BerryFlavor>>,
    pub colors: HashMap<ColorId, Arc<Color>>,
    pub contest_effects: HashMap<ContestEffectId, Arc<ContestEffect>>,
    pub contest_types: HashMap<ContestTypeId, Arc<ContestType>>,
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
    pub move_targets: HashMap<PokemonMoveTargetId, Arc<PokemonMoveTarget>>,
    pub pokemon: HashMap<PokemonId, Arc<Pokemon>>,
    pub pokemon_types: HashMap<PokemonTypeId, Arc<PokemonType>>,
    pub pokemon_type_efficacies: PokemonTypeEfficaciesCollection,
    pub regions: HashMap<RegionId, Arc<Region>>,
    pub shapes: HashMap<ShapeId, Arc<Shape>>,
    pub species: HashMap<SpeciesId, Arc<Species>>,
    pub super_contest_effects: HashMap<SuperContestEffectId, Arc<SuperContestEffect>>,
    pub versions: HashMap<VersionId, Arc<Version>>,
    pub version_groups: HashMap<VersionGroupId, Arc<VersionGroup>>,
}

impl LinkContext {
    pub fn build_data(self) -> PokeData {
        PokeData {
            abilities: AbilitiesCollection::new(&self),
            berries: BerriesCollection::new(&self),
            egg_groups: EggGroupsCollection::new(&self),
            generations: GenerationsCollection::new(&self),
            items: ItemsCollection::new(&self),
            item_categories: ItemCategoriesCollection::new(&self),
            locations: LocationsCollection::new(&self),
            location_areas: LocationAreasCollection::new(&self),
            pokemon: PokemonCollection::new(&self),
            regions: RegionsCollection::new(&self),
            species: SpeciesCollection::new(&self),
            versions: VersionsCollection::new(&self),
            pokemon_type_efficacies: self.pokemon_type_efficacies,
        }
    }
}
