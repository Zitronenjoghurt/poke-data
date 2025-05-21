use crate::collections::pokemon_type_efficacies::PokemonTypeEfficaciesCollection;
use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::data::PokeData;
use crate::models::ability::{AbilityId, UnlinkedAbility};
use crate::models::berry::{BerryId, UnlinkedBerry};
use crate::models::berry_firmness::{BerryFirmness, BerryFirmnessId};
use crate::models::berry_flavor::{BerryFlavor, BerryFlavorId};
use crate::models::color::{Color, ColorId};
use crate::models::contest_effect::{ContestEffect, ContestEffectId};
use crate::models::contest_type::{ContestType, ContestTypeId};
use crate::models::damage_class::{DamageClass, DamageClassId};
use crate::models::egg_group::{EggGroup, EggGroupId};
use crate::models::encounter::{EncounterId, UnlinkedEncounter};
use crate::models::encounter_condition::{EncounterCondition, EncounterConditionId};
use crate::models::encounter_condition_value::{
    EncounterConditionValueId, UnlinkedEncounterConditionValue,
};
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
use crate::models::pokemon_move::{PokemonMoveId, UnlinkedPokemonMove};
use crate::models::pokemon_move_ailment::{PokemonMoveAilment, PokemonMoveAilmentId};
use crate::models::pokemon_move_category::{PokemonMoveCategory, PokemonMoveCategoryId};
use crate::models::pokemon_move_effect::{PokemonMoveEffect, PokemonMoveEffectId};
use crate::models::pokemon_move_flag::{PokemonMoveFlag, PokemonMoveFlagId};
use crate::models::pokemon_move_target::{PokemonMoveTarget, PokemonMoveTargetId};
use crate::models::pokemon_type::{PokemonTypeId, UnlinkedPokemonType};
use crate::models::pokemon_type_efficacies::{
    PokemonTypeEfficacies, PokemonTypeEfficaciesByGeneration,
};
use crate::models::region::{Region, RegionId};
use crate::models::shape::{Shape, ShapeId};
use crate::models::species::{SpeciesId, UnlinkedSpecies};
use crate::models::super_contest_effect::{SuperContestEffect, SuperContestEffectId};
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
    pub berry_flavors: HashMap<BerryFlavorId, BerryFlavor>,
    pub colors: HashMap<ColorId, Color>,
    pub contest_effects: HashMap<ContestEffectId, ContestEffect>,
    pub contest_types: HashMap<ContestTypeId, ContestType>,
    pub damage_classes: HashMap<DamageClassId, DamageClass>,
    pub encounters: HashMap<EncounterId, UnlinkedEncounter>,
    pub encounter_conditions: HashMap<EncounterConditionId, EncounterCondition>,
    pub encounter_condition_values:
        HashMap<EncounterConditionValueId, UnlinkedEncounterConditionValue>,
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
    pub moves: HashMap<PokemonMoveId, UnlinkedPokemonMove>,
    pub move_ailments: HashMap<PokemonMoveAilmentId, PokemonMoveAilment>,
    pub move_categories: HashMap<PokemonMoveCategoryId, PokemonMoveCategory>,
    pub move_effects: HashMap<PokemonMoveEffectId, PokemonMoveEffect>,
    pub move_flags: HashMap<PokemonMoveFlagId, PokemonMoveFlag>,
    pub move_targets: HashMap<PokemonMoveTargetId, PokemonMoveTarget>,
    pub pokemon: HashMap<PokemonId, UnlinkedPokemon>,
    pub pokemon_types: HashMap<PokemonTypeId, UnlinkedPokemonType>,
    pub pokemon_type_efficacies: PokemonTypeEfficacies,
    pub pokemon_type_efficacies_past: PokemonTypeEfficaciesByGeneration,
    pub regions: HashMap<RegionId, Region>,
    pub shapes: HashMap<ShapeId, Shape>,
    pub species: HashMap<SpeciesId, UnlinkedSpecies>,
    pub super_contest_effects: HashMap<SuperContestEffectId, SuperContestEffect>,
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
        let mut context = LinkContext::default();
        context.berry_firmnesses = self.berry_firmnesses.clone().into_arc_map();
        context.berry_flavors = self.berry_flavors.clone().into_arc_map();
        context.colors = self.colors.clone().into_arc_map();
        context.contest_effects = self.contest_effects.clone().into_arc_map();
        context.contest_types = self.contest_types.clone().into_arc_map();
        context.damage_classes = self.damage_classes.clone().into_arc_map();
        context.encounter_conditions = self.encounter_conditions.clone().into_arc_map();
        context.encounter_methods = self.encounter_methods.clone().into_arc_map();
        context.egg_groups = self.egg_groups.clone().into_arc_map();
        context.evolution_triggers = self.evolution_triggers.clone().into_arc_map();
        context.growth_rates = self.growth_rates.clone().into_arc_map();
        context.habitats = self.habitats.clone().into_arc_map();
        context.item_flags = self.item_flags.clone().into_arc_map();
        context.item_fling_effects = self.item_fling_effects.clone().into_arc_map();
        context.item_pockets = self.item_pockets.clone().into_arc_map();
        context.move_ailments = self.move_ailments.clone().into_arc_map();
        context.move_categories = self.move_categories.clone().into_arc_map();
        context.move_effects = self.move_effects.clone().into_arc_map();
        context.move_flags = self.move_flags.clone().into_arc_map();
        context.move_targets = self.move_targets.clone().into_arc_map();
        context.regions = self.regions.clone().into_arc_map();
        context.shapes = self.shapes.clone().into_arc_map();
        context.super_contest_effects = self.super_contest_effects.clone().into_arc_map();
        context.pokemon_type_efficacies =
            PokemonTypeEfficaciesCollection::build_from_unlinked(self);

        context.item_categories = self.item_categories.link(&context);
        context.items = self.items.link(&context);

        context.berries = self.berries.link(&context);

        context.generations = self.generations.link(&context);

        context.version_groups = self.version_groups.link(&context);
        context.versions = self.versions.link(&context);

        context.moves = self.moves.link(&context);

        context.encounter_condition_values = self.encounter_condition_values.link(&context);

        context.encounters = self.encounters.link(&context);

        context.locations = self.locations.link(&context);
        context.location_areas = self.location_areas.link(&context);

        context.abilities = self.abilities.link(&context);
        context.pokemon_types = self.pokemon_types.link(&context);

        context.species = self.species.link(&context);

        context.pokemon = self.pokemon.link(&context);

        context.build_data()
    }
}
