use crate::models::poke_api::ability::AbilityData;
use crate::models::poke_api::ability_changelog::{AbilityChangelogData, AbilityChangelogId};
use crate::models::poke_api::ability_changelog_prose::AbilityChangelogProseData;
use crate::models::poke_api::ability_flavor_text::AbilityFlavorTextData;
use crate::models::poke_api::ability_names::AbilityNameData;
use crate::models::poke_api::ability_prose::AbilityProseData;
use crate::models::poke_api::berries::BerryData;
use crate::models::poke_api::berry_firmness::BerryFirmnessData;
use crate::models::poke_api::berry_firmness_names::BerryFirmnessNameData;
use crate::models::poke_api::egg_group::EggGroupData;
use crate::models::poke_api::egg_group_prose::EggGroupProseData;
use crate::models::poke_api::encounter_method_prose::EncounterMethodProseData;
use crate::models::poke_api::encounter_methods::EncounterMethodData;
use crate::models::poke_api::evolution_chains::{EvolutionChainData, EvolutionChainId};
use crate::models::poke_api::evolution_trigger_prose::EvolutionTriggerProseData;
use crate::models::poke_api::evolution_triggers::EvolutionTriggerData;
use crate::models::poke_api::experience::ExperienceData;
use crate::models::poke_api::generation::GenerationData;
use crate::models::poke_api::generation_names::GenerationNameData;
use crate::models::poke_api::growth_rate_prose::GrowthRateProseData;
use crate::models::poke_api::growth_rates::GrowthRateData;
use crate::models::poke_api::item_flavor_text::ItemFlavorTextData;
use crate::models::poke_api::item_names::ItemNameData;
use crate::models::poke_api::item_prose::ItemProseData;
use crate::models::poke_api::items::ItemData;
use crate::models::poke_api::location_area_encounter_rates::LocationAreaEncounterRateData;
use crate::models::poke_api::location_area_prose::LocationAreaProseData;
use crate::models::poke_api::location_areas::LocationAreaData;
use crate::models::poke_api::location_game_indices::LocationGameIndexData;
use crate::models::poke_api::location_names::LocationNameData;
use crate::models::poke_api::locations::LocationData;
use crate::models::poke_api::pokemon::PokemonData;
use crate::models::poke_api::pokemon_abilities::PokemonAbilityData;
use crate::models::poke_api::pokemon_color_names::ColorNameData;
use crate::models::poke_api::pokemon_colors::ColorData;
use crate::models::poke_api::pokemon_habitat_names::HabitatNameData;
use crate::models::poke_api::pokemon_habitats::HabitatData;
use crate::models::poke_api::pokemon_shape_prose::ShapeProseData;
use crate::models::poke_api::pokemon_shapes::ShapeData;
use crate::models::poke_api::pokemon_species::PokemonSpeciesData;
use crate::models::poke_api::region::RegionData;
use crate::models::poke_api::region_names::RegionNameData;
use crate::models::poke_api::version_group_pokemon_move_methods::VersionGroupMoveMethodData;
use crate::models::poke_api::version_group_regions::VersionGroupRegionData;
use crate::models::poke_api::version_groups::VersionGroupData;
use crate::models::poke_api::version_names::VersionNameData;
use crate::models::poke_api::versions::VersionData;
use crate::models::RemoteModel;
use crate::traits::has_id::IntoIdMap;
use crate::traits::into_model::IntoModel;
use poke_data::data::unlinked::UnlinkedPokeData;
use poke_data::models::ability::AbilityId;
use poke_data::models::berry::BerryId;
use poke_data::models::berry_firmness::BerryFirmnessId;
use poke_data::models::color::ColorId;
use poke_data::models::egg_group::EggGroupId;
use poke_data::models::encounter_method::EncounterMethodId;
use poke_data::models::evolution_trigger::EvolutionTriggerId;
use poke_data::models::generation::GenerationId;
use poke_data::models::growth_rate::GrowthRateId;
use poke_data::models::habitat::HabitatId;
use poke_data::models::item::ItemId;
use poke_data::models::location::LocationId;
use poke_data::models::location_area::LocationAreaId;
use poke_data::models::pokemon::PokemonId;
use poke_data::models::region::RegionId;
use poke_data::models::shape::ShapeId;
use poke_data::models::species::SpeciesId;
use poke_data::models::version::VersionId;
use poke_data::models::version_group::VersionGroupId;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

mod models;
mod traits;

#[derive(Debug)]
pub struct RawData {
    pub abilities: HashMap<AbilityId, AbilityData>,
    pub ability_changelogs: HashMap<AbilityId, Vec<AbilityChangelogData>>,
    pub ability_changelog_prose: HashMap<AbilityChangelogId, Vec<AbilityChangelogProseData>>,
    pub ability_flavor_texts: HashMap<AbilityId, Vec<AbilityFlavorTextData>>,
    pub ability_names: HashMap<AbilityId, Vec<AbilityNameData>>,
    pub ability_prose: HashMap<AbilityId, Vec<AbilityProseData>>,
    pub berries: HashMap<BerryId, BerryData>,
    pub berry_firmnesses: HashMap<BerryFirmnessId, BerryFirmnessData>,
    pub berry_firmness_names: HashMap<BerryFirmnessId, Vec<BerryFirmnessNameData>>,
    pub colors: HashMap<ColorId, ColorData>,
    pub color_names: HashMap<ColorId, Vec<ColorNameData>>,
    pub encounter_methods: HashMap<EncounterMethodId, EncounterMethodData>,
    pub encounter_method_prose: HashMap<EncounterMethodId, Vec<EncounterMethodProseData>>,
    pub egg_groups: HashMap<EggGroupId, EggGroupData>,
    pub egg_group_prose: HashMap<EggGroupId, Vec<EggGroupProseData>>,
    pub evolution_chains: HashMap<EvolutionChainId, EvolutionChainData>,
    pub evolution_triggers: HashMap<EvolutionTriggerId, EvolutionTriggerData>,
    pub evolution_trigger_prose: HashMap<EvolutionTriggerId, Vec<EvolutionTriggerProseData>>,
    pub experiences: HashMap<GrowthRateId, Vec<ExperienceData>>,
    pub generations: HashMap<GenerationId, GenerationData>,
    pub generation_names: HashMap<GenerationId, Vec<GenerationNameData>>,
    pub growth_rates: HashMap<GrowthRateId, GrowthRateData>,
    pub growth_rate_prose: HashMap<GrowthRateId, Vec<GrowthRateProseData>>,
    pub habitats: HashMap<HabitatId, HabitatData>,
    pub habitat_names: HashMap<HabitatId, Vec<HabitatNameData>>,
    pub items: HashMap<ItemId, ItemData>,
    pub item_flavor_texts: HashMap<ItemId, Vec<ItemFlavorTextData>>,
    pub item_names: HashMap<ItemId, Vec<ItemNameData>>,
    pub item_prose: HashMap<ItemId, Vec<ItemProseData>>,
    pub locations: HashMap<LocationId, LocationData>,
    pub location_names: HashMap<LocationId, Vec<LocationNameData>>,
    pub location_game_indices: HashMap<LocationId, Vec<LocationGameIndexData>>,
    pub location_areas: HashMap<LocationAreaId, LocationAreaData>,
    pub location_area_encounter_rates: HashMap<LocationAreaId, Vec<LocationAreaEncounterRateData>>,
    pub location_area_prose: HashMap<LocationAreaId, Vec<LocationAreaProseData>>,
    pub pokemon: HashMap<PokemonId, PokemonData>,
    pub pokemon_abilities: HashMap<PokemonId, Vec<PokemonAbilityData>>,
    pub regions: HashMap<RegionId, RegionData>,
    pub region_names: HashMap<RegionId, Vec<RegionNameData>>,
    pub shapes: HashMap<ShapeId, ShapeData>,
    pub shape_prose: HashMap<ShapeId, Vec<ShapeProseData>>,
    pub species: HashMap<SpeciesId, PokemonSpeciesData>,
    pub version_groups: HashMap<VersionGroupId, VersionGroupData>,
    pub version_group_move_methods: HashMap<VersionGroupId, Vec<VersionGroupMoveMethodData>>,
    pub version_group_regions: HashMap<VersionGroupId, Vec<VersionGroupRegionData>>,
    pub versions: HashMap<VersionId, VersionData>,
    pub version_names: HashMap<VersionId, Vec<VersionNameData>>,
}

impl RawData {
    pub async fn load(base_path: &Path) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            abilities: AbilityData::load(base_path).await?.into_id_map(),
            ability_changelogs: AbilityChangelogData::load(base_path)
                .await?
                .into_id_map_grouped(),
            ability_changelog_prose: AbilityChangelogProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            ability_flavor_texts: AbilityFlavorTextData::load(base_path)
                .await?
                .into_id_map_grouped(),
            ability_names: AbilityNameData::load(base_path)
                .await?
                .into_id_map_grouped(),
            ability_prose: AbilityProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            berries: BerryData::load(base_path).await?.into_id_map(),
            berry_firmnesses: BerryFirmnessData::load(base_path).await?.into_id_map(),
            berry_firmness_names: BerryFirmnessNameData::load(base_path)
                .await?
                .into_id_map_grouped(),
            colors: ColorData::load(base_path).await?.into_id_map(),
            color_names: ColorNameData::load(base_path).await?.into_id_map_grouped(),
            encounter_methods: EncounterMethodData::load(base_path).await?.into_id_map(),
            encounter_method_prose: EncounterMethodProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            egg_groups: EggGroupData::load(base_path).await?.into_id_map(),
            egg_group_prose: EggGroupProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            evolution_chains: EvolutionChainData::load(base_path).await?.into_id_map(),
            evolution_triggers: EvolutionTriggerData::load(base_path).await?.into_id_map(),
            evolution_trigger_prose: EvolutionTriggerProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            experiences: ExperienceData::load(base_path).await?.into_id_map_grouped(),
            generations: GenerationData::load(base_path).await?.into_id_map(),
            generation_names: GenerationNameData::load(base_path)
                .await?
                .into_id_map_grouped(),
            growth_rates: GrowthRateData::load(base_path).await?.into_id_map(),
            growth_rate_prose: GrowthRateProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            habitats: HabitatData::load(base_path).await?.into_id_map(),
            habitat_names: HabitatNameData::load(base_path)
                .await?
                .into_id_map_grouped(),
            items: ItemData::load(base_path).await?.into_id_map(),
            item_flavor_texts: ItemFlavorTextData::load(base_path)
                .await?
                .into_id_map_grouped(),
            item_names: ItemNameData::load(base_path).await?.into_id_map_grouped(),
            item_prose: ItemProseData::load(base_path).await?.into_id_map_grouped(),
            locations: LocationData::load(base_path).await?.into_id_map(),
            location_names: LocationNameData::load(base_path)
                .await?
                .into_id_map_grouped(),
            location_game_indices: LocationGameIndexData::load(base_path)
                .await?
                .into_id_map_grouped(),
            location_areas: LocationAreaData::load(base_path).await?.into_id_map(),
            location_area_encounter_rates: LocationAreaEncounterRateData::load(base_path)
                .await?
                .into_id_map_grouped(),
            location_area_prose: LocationAreaProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokemon: PokemonData::load(base_path).await?.into_id_map(),
            pokemon_abilities: PokemonAbilityData::load(base_path)
                .await?
                .into_id_map_grouped(),
            regions: RegionData::load(base_path).await?.into_id_map(),
            region_names: RegionNameData::load(base_path).await?.into_id_map_grouped(),
            shapes: ShapeData::load(base_path).await?.into_id_map(),
            shape_prose: ShapeProseData::load(base_path).await?.into_id_map_grouped(),
            species: PokemonSpeciesData::load(base_path).await?.into_id_map(),
            version_groups: VersionGroupData::load(base_path).await?.into_id_map(),
            version_group_move_methods: VersionGroupMoveMethodData::load(base_path)
                .await?
                .into_id_map_grouped(),
            version_group_regions: VersionGroupRegionData::load(base_path)
                .await?
                .into_id_map_grouped(),
            versions: VersionData::load(base_path).await?.into_id_map(),
            version_names: VersionNameData::load(base_path)
                .await?
                .into_id_map_grouped(),
        })
    }

    pub fn parse(&self) -> UnlinkedPokeData {
        UnlinkedPokeData {
            abilities: self.abilities.clone().into_model(self),
            berries: self.berries.clone().into_model(self),
            berry_firmnesses: self.berry_firmnesses.clone().into_model(self),
            colors: self.colors.clone().into_model(self),
            encounter_methods: self.encounter_methods.clone().into_model(self),
            egg_groups: self.egg_groups.clone().into_model(self),
            evolution_triggers: self.evolution_triggers.clone().into_model(self),
            generations: self.generations.clone().into_model(self),
            growth_rates: self.growth_rates.clone().into_model(self),
            habitats: self.habitats.clone().into_model(self),
            items: self.items.clone().into_model(self),
            locations: self.locations.clone().into_model(self),
            location_areas: self.location_areas.clone().into_model(self),
            pokemon: self.pokemon.clone().into_model(self),
            regions: self.regions.clone().into_model(self),
            shapes: self.shapes.clone().into_model(self),
            species: self.species.clone().into_model(self),
            version_groups: self.version_groups.clone().into_model(self),
            versions: self.versions.clone().into_model(self),
        }
    }
}
