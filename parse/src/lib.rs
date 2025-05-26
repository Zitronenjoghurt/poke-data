use crate::models::poke_api::ability::AbilityData;
use crate::models::poke_api::ability_changelog::{AbilityChangelogData, AbilityChangelogId};
use crate::models::poke_api::ability_changelog_prose::AbilityChangelogProseData;
use crate::models::poke_api::ability_flavor_text::AbilityFlavorTextData;
use crate::models::poke_api::ability_names::AbilityNameData;
use crate::models::poke_api::ability_prose::AbilityProseData;
use crate::models::poke_api::berries::BerryData;
use crate::models::poke_api::berry_firmness::BerryFirmnessData;
use crate::models::poke_api::berry_firmness_names::BerryFirmnessNameData;
use crate::models::poke_api::berry_flavors::BerryFlavorData;
use crate::models::poke_api::contest_effect_prose::ContestEffectProseData;
use crate::models::poke_api::contest_effects::ContestEffectData;
use crate::models::poke_api::contest_type_names::ContestTypeNameData;
use crate::models::poke_api::contest_types::ContestTypeData;
use crate::models::poke_api::egg_group::EggGroupData;
use crate::models::poke_api::egg_group_prose::EggGroupProseData;
use crate::models::poke_api::encounter::EncounterData;
use crate::models::poke_api::encounter_condition_prose::EncounterConditionProseData;
use crate::models::poke_api::encounter_condition_value_map::{
    EncounterConditionValueMapData, EncounterSlotId,
};
use crate::models::poke_api::encounter_condition_value_prose::EncounterConditionValueProseData;
use crate::models::poke_api::encounter_condition_values::EncounterConditionValueData;
use crate::models::poke_api::encounter_conditions::EncounterConditionData;
use crate::models::poke_api::encounter_method_prose::EncounterMethodProseData;
use crate::models::poke_api::encounter_methods::EncounterMethodData;
use crate::models::poke_api::encounter_slots::EncounterSlotData;
use crate::models::poke_api::evolution_chains::{EvolutionChainData, EvolutionChainId};
use crate::models::poke_api::evolution_trigger_prose::EvolutionTriggerProseData;
use crate::models::poke_api::evolution_triggers::EvolutionTriggerData;
use crate::models::poke_api::evolutions::EvolutionData;
use crate::models::poke_api::experience::ExperienceData;
use crate::models::poke_api::generation::GenerationData;
use crate::models::poke_api::generation_names::GenerationNameData;
use crate::models::poke_api::growth_rate_prose::GrowthRateProseData;
use crate::models::poke_api::growth_rates::GrowthRateData;
use crate::models::poke_api::item_categories::ItemCategoryData;
use crate::models::poke_api::item_category_prose::ItemCategoryProseData;
use crate::models::poke_api::item_flag_map::ItemFlagMapData;
use crate::models::poke_api::item_flag_prose::ItemFlagProseData;
use crate::models::poke_api::item_flags::ItemFlagData;
use crate::models::poke_api::item_flavor_text::ItemFlavorTextData;
use crate::models::poke_api::item_fling_effect_prose::ItemFlingEffectProseData;
use crate::models::poke_api::item_fling_effects::ItemFlingEffectData;
use crate::models::poke_api::item_game_indices::ItemGameIndexData;
use crate::models::poke_api::item_names::ItemNameData;
use crate::models::poke_api::item_pocket_names::ItemPocketNameData;
use crate::models::poke_api::item_pockets::ItemPocketData;
use crate::models::poke_api::item_prose::ItemProseData;
use crate::models::poke_api::items::ItemData;
use crate::models::poke_api::location_area_encounter_rates::LocationAreaEncounterRateData;
use crate::models::poke_api::location_area_prose::LocationAreaProseData;
use crate::models::poke_api::location_areas::LocationAreaData;
use crate::models::poke_api::location_game_indices::LocationGameIndexData;
use crate::models::poke_api::location_names::LocationNameData;
use crate::models::poke_api::locations::LocationData;
use crate::models::poke_api::move_ailment_names::MoveAilmentNameData;
use crate::models::poke_api::move_ailments::MoveAilmentData;
use crate::models::poke_api::move_categories::MoveCategoryData;
use crate::models::poke_api::move_category_prose::MoveCategoryProseData;
use crate::models::poke_api::move_changelogs::MoveChangelogData;
use crate::models::poke_api::move_damage_class_prose::DamageClassProseData;
use crate::models::poke_api::move_damage_classes::DamageClassData;
use crate::models::poke_api::move_effect_changelog::{
    MoveEffectChangelogData, MoveEffectChangelogId,
};
use crate::models::poke_api::move_effect_changelog_prose::MoveEffectChangelogProseData;
use crate::models::poke_api::move_effect_prose::MoveEffectProseData;
use crate::models::poke_api::move_effects::MoveEffectData;
use crate::models::poke_api::move_flag_map::MoveFlagMapData;
use crate::models::poke_api::move_flag_prose::MoveFlagProseData;
use crate::models::poke_api::move_flags::MoveFlagData;
use crate::models::poke_api::move_flavor_texts::MoveFlavorTextData;
use crate::models::poke_api::move_meta::MoveMetaData;
use crate::models::poke_api::move_meta_stat_changes::MoveMetaStatChangeData;
use crate::models::poke_api::move_method_prose::MoveMethodProseData;
use crate::models::poke_api::move_methods::MoveMethodData;
use crate::models::poke_api::move_names::MoveNameData;
use crate::models::poke_api::move_target_prose::MoveTargetProseData;
use crate::models::poke_api::move_targets::MoveTargetData;
use crate::models::poke_api::moves::MoveData;
use crate::models::poke_api::pokedex_prose::PokedexProseData;
use crate::models::poke_api::pokedex_version_groups::PokedexVersionGroupData;
use crate::models::poke_api::pokedexes::PokedexData;
use crate::models::poke_api::pokemon::PokemonData;
use crate::models::poke_api::pokemon_abilities::PokemonAbilityData;
use crate::models::poke_api::pokemon_color_names::ColorNameData;
use crate::models::poke_api::pokemon_colors::ColorData;
use crate::models::poke_api::pokemon_dex_numbers::PokemonDexNumberData;
use crate::models::poke_api::pokemon_egg_groups::PokemonEggGroupData;
use crate::models::poke_api::pokemon_form_names::PokemonFormNameData;
use crate::models::poke_api::pokemon_form_pokeathlon_stats::PokemonFormPokeathlonStatData;
use crate::models::poke_api::pokemon_form_types::PokemonFormTypeData;
use crate::models::poke_api::pokemon_forms::PokemonFormData;
use crate::models::poke_api::pokemon_habitat_names::HabitatNameData;
use crate::models::poke_api::pokemon_habitats::HabitatData;
use crate::models::poke_api::pokemon_items::PokemonItemData;
use crate::models::poke_api::pokemon_move_map::PokemonMoveMapData;
use crate::models::poke_api::pokemon_shape_prose::ShapeProseData;
use crate::models::poke_api::pokemon_shapes::ShapeData;
use crate::models::poke_api::pokemon_species::PokemonSpeciesData;
use crate::models::poke_api::pokemon_species_flavor_texts::PokemonSpeciesFlavorTextData;
use crate::models::poke_api::pokemon_species_names::PokemonSpeciesNameData;
use crate::models::poke_api::pokemon_species_prose::PokemonSpeciesProseData;
use crate::models::poke_api::pokemon_stats::PokemonStatData;
use crate::models::poke_api::pokemon_type_efficacy::PokemonTypeEfficacyData;
use crate::models::poke_api::pokemon_type_efficacy_past::PokemonTypeEfficacyPastData;
use crate::models::poke_api::pokemon_type_game_indices::PokemonTypeGameIndexData;
use crate::models::poke_api::pokemon_type_names::PokemonTypeNameData;
use crate::models::poke_api::pokemon_types::PokemonTypeData;
use crate::models::poke_api::pokemon_types_map::PokemonTypeMapData;
use crate::models::poke_api::pokemon_types_past_map::PokemonTypePastMapData;
use crate::models::poke_api::region::RegionData;
use crate::models::poke_api::region_names::RegionNameData;
use crate::models::poke_api::super_contest_effect_prose::SuperContestEffectProseData;
use crate::models::poke_api::super_contest_effects::SuperContestEffectData;
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
use poke_data::models::berry_flavor::{BerryFlavor, BerryFlavorId};
use poke_data::models::color::ColorId;
use poke_data::models::contest_effect::ContestEffectId;
use poke_data::models::contest_type::ContestTypeId;
use poke_data::models::damage_class::DamageClassId;
use poke_data::models::egg_group::EggGroupId;
use poke_data::models::encounter::EncounterId;
use poke_data::models::encounter_condition::EncounterConditionId;
use poke_data::models::encounter_condition_value::EncounterConditionValueId;
use poke_data::models::encounter_method::EncounterMethodId;
use poke_data::models::evolution_trigger::EvolutionTriggerId;
use poke_data::models::generation::GenerationId;
use poke_data::models::growth_rate::GrowthRateId;
use poke_data::models::habitat::HabitatId;
use poke_data::models::item::ItemId;
use poke_data::models::item_category::ItemCategoryId;
use poke_data::models::item_flag::ItemFlagId;
use poke_data::models::item_fling_effect::ItemFlingEffectId;
use poke_data::models::item_pocket::ItemPocketId;
use poke_data::models::localized_names::LocalizedStrings;
use poke_data::models::location::LocationId;
use poke_data::models::location_area::LocationAreaId;
use poke_data::models::pokedex::PokedexId;
use poke_data::models::pokemon::PokemonId;
use poke_data::models::pokemon_form::PokemonFormId;
use poke_data::models::pokemon_move::PokemonMoveId;
use poke_data::models::pokemon_move_ailment::PokemonMoveAilmentId;
use poke_data::models::pokemon_move_category::PokemonMoveCategoryId;
use poke_data::models::pokemon_move_effect::PokemonMoveEffectId;
use poke_data::models::pokemon_move_flag::PokemonMoveFlagId;
use poke_data::models::pokemon_move_method::PokemonMoveMethodId;
use poke_data::models::pokemon_move_target::PokemonMoveTargetId;
use poke_data::models::pokemon_type::PokemonTypeId;
use poke_data::models::region::RegionId;
use poke_data::models::shape::ShapeId;
use poke_data::models::species::SpeciesId;
use poke_data::models::super_contest_effect::SuperContestEffectId;
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
    pub berry_flavors: HashMap<BerryId, Vec<BerryFlavorData>>,
    pub colors: HashMap<ColorId, ColorData>,
    pub color_names: HashMap<ColorId, Vec<ColorNameData>>,
    pub contest_effects: HashMap<ContestEffectId, ContestEffectData>,
    pub contest_effect_prose: HashMap<ContestEffectId, Vec<ContestEffectProseData>>,
    pub contest_types: HashMap<ContestTypeId, ContestTypeData>,
    pub contest_type_names: HashMap<ContestTypeId, Vec<ContestTypeNameData>>,
    pub damage_classes: HashMap<DamageClassId, DamageClassData>,
    pub damage_class_prose: HashMap<DamageClassId, Vec<DamageClassProseData>>,
    pub encounters: HashMap<EncounterId, EncounterData>,
    pub encounter_conditions: HashMap<EncounterConditionId, EncounterConditionData>,
    pub encounter_condition_prose: HashMap<EncounterConditionId, Vec<EncounterConditionProseData>>,
    pub encounter_condition_values: HashMap<EncounterConditionValueId, EncounterConditionValueData>,
    pub encounter_condition_value_prose:
        HashMap<EncounterConditionValueId, Vec<EncounterConditionValueProseData>>,
    pub encounter_condition_value_map: HashMap<EncounterId, Vec<EncounterConditionValueMapData>>,
    pub encounter_methods: HashMap<EncounterMethodId, EncounterMethodData>,
    pub encounter_method_prose: HashMap<EncounterMethodId, Vec<EncounterMethodProseData>>,
    pub encounter_slots: HashMap<EncounterSlotId, EncounterSlotData>,
    pub egg_groups: HashMap<EggGroupId, EggGroupData>,
    pub egg_group_prose: HashMap<EggGroupId, Vec<EggGroupProseData>>,
    pub evolutions: HashMap<SpeciesId, Vec<EvolutionData>>,
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
    pub item_categories: HashMap<ItemCategoryId, ItemCategoryData>,
    pub item_category_prose: HashMap<ItemCategoryId, Vec<ItemCategoryProseData>>,
    pub item_flags: HashMap<ItemFlagId, ItemFlagData>,
    pub item_flag_map: HashMap<ItemId, Vec<ItemFlagMapData>>,
    pub item_flag_prose: HashMap<ItemFlagId, Vec<ItemFlagProseData>>,
    pub item_fling_effects: HashMap<ItemFlingEffectId, ItemFlingEffectData>,
    pub item_fling_effect_prose: HashMap<ItemFlingEffectId, Vec<ItemFlingEffectProseData>>,
    pub item_game_indices: HashMap<ItemId, Vec<ItemGameIndexData>>,
    pub item_pockets: HashMap<ItemPocketId, ItemPocketData>,
    pub item_pocket_names: HashMap<ItemPocketId, Vec<ItemPocketNameData>>,
    pub locations: HashMap<LocationId, LocationData>,
    pub location_names: HashMap<LocationId, Vec<LocationNameData>>,
    pub location_game_indices: HashMap<LocationId, Vec<LocationGameIndexData>>,
    pub location_areas: HashMap<LocationAreaId, LocationAreaData>,
    pub location_area_encounter_rates: HashMap<LocationAreaId, Vec<LocationAreaEncounterRateData>>,
    pub location_area_prose: HashMap<LocationAreaId, Vec<LocationAreaProseData>>,
    pub moves: HashMap<PokemonMoveId, MoveData>,
    pub move_changelogs: HashMap<PokemonMoveId, Vec<MoveChangelogData>>,
    pub move_flag_map: HashMap<PokemonMoveId, Vec<MoveFlagMapData>>,
    pub move_flavor_texts: HashMap<PokemonMoveId, Vec<MoveFlavorTextData>>,
    pub move_meta: HashMap<PokemonMoveId, MoveMetaData>,
    pub move_meta_stat_changes: HashMap<PokemonMoveId, Vec<MoveMetaStatChangeData>>,
    pub move_names: HashMap<PokemonMoveId, Vec<MoveNameData>>,
    pub move_ailments: HashMap<PokemonMoveAilmentId, MoveAilmentData>,
    pub move_ailment_names: HashMap<PokemonMoveAilmentId, Vec<MoveAilmentNameData>>,
    pub move_categories: HashMap<PokemonMoveCategoryId, MoveCategoryData>,
    pub move_category_prose: HashMap<PokemonMoveCategoryId, Vec<MoveCategoryProseData>>,
    pub move_effects: HashMap<PokemonMoveEffectId, MoveEffectData>,
    pub move_effect_prose: HashMap<PokemonMoveEffectId, Vec<MoveEffectProseData>>,
    pub move_effect_changelogs: HashMap<PokemonMoveEffectId, Vec<MoveEffectChangelogData>>,
    pub move_effect_changelog_prose:
        HashMap<MoveEffectChangelogId, Vec<MoveEffectChangelogProseData>>,
    pub move_flags: HashMap<PokemonMoveFlagId, MoveFlagData>,
    pub move_flag_prose: HashMap<PokemonMoveFlagId, Vec<MoveFlagProseData>>,
    pub move_methods: HashMap<PokemonMoveMethodId, MoveMethodData>,
    pub move_method_prose: HashMap<PokemonMoveMethodId, Vec<MoveMethodProseData>>,
    pub move_targets: HashMap<PokemonMoveTargetId, MoveTargetData>,
    pub move_target_prose: HashMap<PokemonMoveTargetId, Vec<MoveTargetProseData>>,
    pub pokedexes: HashMap<PokedexId, PokedexData>,
    pub pokedex_prose: HashMap<PokedexId, Vec<PokedexProseData>>,
    pub pokedex_version_groups: HashMap<PokedexId, Vec<PokedexVersionGroupData>>,
    pub pokemon_dex_numbers: HashMap<PokedexId, Vec<PokemonDexNumberData>>,
    pub pokemon: HashMap<PokemonId, PokemonData>,
    pub pokemon_egg_groups: HashMap<SpeciesId, Vec<PokemonEggGroupData>>,
    pub pokemon_forms: HashMap<PokemonFormId, PokemonFormData>,
    pub pokemon_form_names: HashMap<PokemonFormId, Vec<PokemonFormNameData>>,
    pub pokemon_form_pokeathlon_stats: HashMap<PokemonFormId, Vec<PokemonFormPokeathlonStatData>>,
    pub pokemon_form_types: HashMap<PokemonFormId, Vec<PokemonFormTypeData>>,
    pub pokemon_form_id_map: HashMap<PokemonId, Vec<PokemonFormId>>,
    pub pokemon_items: HashMap<PokemonId, Vec<PokemonItemData>>,
    pub pokemon_move_map: HashMap<PokemonId, Vec<PokemonMoveMapData>>,
    pub pokemon_stats: HashMap<PokemonId, Vec<PokemonStatData>>,
    pub pokemon_types: HashMap<PokemonTypeId, PokemonTypeData>,
    pub pokemon_types_map: HashMap<PokemonId, Vec<PokemonTypeMapData>>,
    pub pokemon_types_past_map: HashMap<PokemonId, Vec<PokemonTypePastMapData>>,
    pub pokemon_type_names: HashMap<PokemonTypeId, Vec<PokemonTypeNameData>>,
    pub pokemon_type_game_indices: HashMap<PokemonTypeId, Vec<PokemonTypeGameIndexData>>,
    pub pokemon_type_efficacies: Vec<PokemonTypeEfficacyData>,
    pub pokemon_type_efficacies_past: Vec<PokemonTypeEfficacyPastData>,
    pub pokemon_abilities: HashMap<PokemonId, Vec<PokemonAbilityData>>,
    pub regions: HashMap<RegionId, RegionData>,
    pub region_names: HashMap<RegionId, Vec<RegionNameData>>,
    pub shapes: HashMap<ShapeId, ShapeData>,
    pub shape_prose: HashMap<ShapeId, Vec<ShapeProseData>>,
    pub species: HashMap<SpeciesId, PokemonSpeciesData>,
    pub species_flavor_texts: HashMap<SpeciesId, Vec<PokemonSpeciesFlavorTextData>>,
    pub species_names: HashMap<SpeciesId, Vec<PokemonSpeciesNameData>>,
    pub species_prose: HashMap<SpeciesId, Vec<PokemonSpeciesProseData>>,
    pub super_contest_effects: HashMap<SuperContestEffectId, SuperContestEffectData>,
    pub super_contest_effect_prose: HashMap<SuperContestEffectId, Vec<SuperContestEffectProseData>>,
    pub version_groups: HashMap<VersionGroupId, VersionGroupData>,
    pub version_group_move_methods: HashMap<VersionGroupId, Vec<VersionGroupMoveMethodData>>,
    pub version_group_regions: HashMap<VersionGroupId, Vec<VersionGroupRegionData>>,
    pub versions: HashMap<VersionId, VersionData>,
    pub version_names: HashMap<VersionId, Vec<VersionNameData>>,
}

impl RawData {
    pub async fn load(base_path: &Path) -> Result<Self, Box<dyn Error>> {
        let species = PokemonSpeciesData::load(base_path).await?.into_id_map();
        let evolutions_raw = EvolutionData::load(base_path).await?;
        let evolutions = evolutions_raw
            .iter()
            .map(|evolution| {
                // Map evolution data to the species they are relevant to
                let evolves_into = species.get(&evolution.evolved_species_id).unwrap();
                let evolves_from_id = evolves_into.evolves_from_species_id.unwrap();
                (evolves_from_id, evolution.clone())
            })
            .fold(
                HashMap::new(),
                |mut acc: HashMap<SpeciesId, Vec<EvolutionData>>, (evolves_from_id, evolution)| {
                    acc.entry(evolves_from_id).or_default().push(evolution);
                    acc
                },
            );

        let pokemon_forms = PokemonFormData::load(base_path).await?.into_id_map();
        let pokemon_form_id_map = pokemon_forms
            .iter()
            .map(|(form_id, entry)| {
                // Map pokemon forms to their respective pokemon
                (entry.pokemon_id, form_id)
            })
            .fold(
                HashMap::new(),
                |mut acc: HashMap<SpeciesId, Vec<PokemonFormId>>, (pokemon_id, form_id)| {
                    acc.entry(pokemon_id).or_default().push(*form_id);
                    acc
                },
            );

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
            berry_flavors: BerryFlavorData::load(base_path)
                .await?
                .into_id_map_grouped(),
            colors: ColorData::load(base_path).await?.into_id_map(),
            color_names: ColorNameData::load(base_path).await?.into_id_map_grouped(),
            contest_effects: ContestEffectData::load(base_path).await?.into_id_map(),
            contest_effect_prose: ContestEffectProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            contest_types: ContestTypeData::load(base_path).await?.into_id_map(),
            contest_type_names: ContestTypeNameData::load(base_path)
                .await?
                .into_id_map_grouped(),
            damage_classes: DamageClassData::load(base_path).await?.into_id_map(),
            damage_class_prose: DamageClassProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            encounters: EncounterData::load(base_path).await?.into_id_map(),
            encounter_conditions: EncounterConditionData::load(base_path).await?.into_id_map(),
            encounter_condition_prose: EncounterConditionProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            encounter_condition_values: EncounterConditionValueData::load(base_path)
                .await?
                .into_id_map(),
            encounter_condition_value_prose: EncounterConditionValueProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            encounter_condition_value_map: EncounterConditionValueMapData::load(base_path)
                .await?
                .into_id_map_grouped(),
            encounter_methods: EncounterMethodData::load(base_path).await?.into_id_map(),
            encounter_method_prose: EncounterMethodProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            encounter_slots: EncounterSlotData::load(base_path).await?.into_id_map(),
            egg_groups: EggGroupData::load(base_path).await?.into_id_map(),
            egg_group_prose: EggGroupProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            evolutions,
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
            item_categories: ItemCategoryData::load(base_path).await?.into_id_map(),
            item_category_prose: ItemCategoryProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            item_flags: ItemFlagData::load(base_path).await?.into_id_map(),
            item_flag_map: ItemFlagMapData::load(base_path)
                .await?
                .into_id_map_grouped(),
            item_flag_prose: ItemFlagProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            item_fling_effects: ItemFlingEffectData::load(base_path).await?.into_id_map(),
            item_fling_effect_prose: ItemFlingEffectProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            item_game_indices: ItemGameIndexData::load(base_path)
                .await?
                .into_id_map_grouped(),
            item_pockets: ItemPocketData::load(base_path).await?.into_id_map(),
            item_pocket_names: ItemPocketNameData::load(base_path)
                .await?
                .into_id_map_grouped(),
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
            moves: MoveData::load(base_path).await?.into_id_map(),
            move_changelogs: MoveChangelogData::load(base_path)
                .await?
                .into_id_map_grouped(),
            move_flag_map: MoveFlagMapData::load(base_path)
                .await?
                .into_id_map_grouped(),
            move_flavor_texts: MoveFlavorTextData::load(base_path)
                .await?
                .into_id_map_grouped(),
            move_meta: MoveMetaData::load(base_path).await?.into_id_map(),
            move_meta_stat_changes: MoveMetaStatChangeData::load(base_path)
                .await?
                .into_id_map_grouped(),
            move_names: MoveNameData::load(base_path).await?.into_id_map_grouped(),
            move_ailments: MoveAilmentData::load(base_path).await?.into_id_map(),
            move_ailment_names: MoveAilmentNameData::load(base_path)
                .await?
                .into_id_map_grouped(),
            move_categories: MoveCategoryData::load(base_path).await?.into_id_map(),
            move_category_prose: MoveCategoryProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            move_effects: MoveEffectData::load(base_path).await?.into_id_map(),
            move_effect_prose: MoveEffectProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            move_effect_changelogs: MoveEffectChangelogData::load(base_path)
                .await?
                .into_id_map_grouped(),
            move_effect_changelog_prose: MoveEffectChangelogProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            move_flags: MoveFlagData::load(base_path).await?.into_id_map(),
            move_flag_prose: MoveFlagProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            move_methods: MoveMethodData::load(base_path).await?.into_id_map(),
            move_method_prose: MoveMethodProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            move_targets: MoveTargetData::load(base_path).await?.into_id_map(),
            move_target_prose: MoveTargetProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokedexes: PokedexData::load(base_path).await?.into_id_map(),
            pokedex_prose: PokedexProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokedex_version_groups: PokedexVersionGroupData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokemon_dex_numbers: PokemonDexNumberData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokemon: PokemonData::load(base_path).await?.into_id_map(),
            pokemon_abilities: PokemonAbilityData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokemon_egg_groups: PokemonEggGroupData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokemon_forms,
            pokemon_form_names: PokemonFormNameData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokemon_form_pokeathlon_stats: PokemonFormPokeathlonStatData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokemon_form_types: PokemonFormTypeData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokemon_form_id_map,
            pokemon_items: PokemonItemData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokemon_move_map: PokemonMoveMapData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokemon_stats: PokemonStatData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokemon_types: PokemonTypeData::load(base_path).await?.into_id_map(),
            pokemon_types_map: PokemonTypeMapData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokemon_types_past_map: PokemonTypePastMapData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokemon_type_names: PokemonTypeNameData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokemon_type_game_indices: PokemonTypeGameIndexData::load(base_path)
                .await?
                .into_id_map_grouped(),
            pokemon_type_efficacies: PokemonTypeEfficacyData::load(base_path).await?,
            pokemon_type_efficacies_past: PokemonTypeEfficacyPastData::load(base_path).await?,
            regions: RegionData::load(base_path).await?.into_id_map(),
            region_names: RegionNameData::load(base_path).await?.into_id_map_grouped(),
            shapes: ShapeData::load(base_path).await?.into_id_map(),
            shape_prose: ShapeProseData::load(base_path).await?.into_id_map_grouped(),
            species,
            species_flavor_texts: PokemonSpeciesFlavorTextData::load(base_path)
                .await?
                .into_id_map_grouped(),
            species_names: PokemonSpeciesNameData::load(base_path)
                .await?
                .into_id_map_grouped(),
            species_prose: PokemonSpeciesProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
            super_contest_effects: SuperContestEffectData::load(base_path).await?.into_id_map(),
            super_contest_effect_prose: SuperContestEffectProseData::load(base_path)
                .await?
                .into_id_map_grouped(),
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
            berry_flavors: self.parse_berry_flavors(),
            colors: self.colors.clone().into_model(self),
            contest_effects: self.contest_effects.clone().into_model(self),
            contest_types: self.contest_types.clone().into_model(self),
            damage_classes: self.damage_classes.clone().into_model(self),
            encounters: self.encounters.clone().into_model(self),
            encounter_conditions: self.encounter_conditions.clone().into_model(self),
            encounter_condition_values: self.encounter_condition_values.clone().into_model(self),
            encounter_methods: self.encounter_methods.clone().into_model(self),
            egg_groups: self.egg_groups.clone().into_model(self),
            evolution_triggers: self.evolution_triggers.clone().into_model(self),
            generations: self.generations.clone().into_model(self),
            growth_rates: self.growth_rates.clone().into_model(self),
            habitats: self.habitats.clone().into_model(self),
            items: self.items.clone().into_model(self),
            item_categories: self.item_categories.clone().into_model(self),
            item_flags: self.item_flags.clone().into_model(self),
            item_fling_effects: self.item_fling_effects.clone().into_model(self),
            item_pockets: self.item_pockets.clone().into_model(self),
            locations: self.locations.clone().into_model(self),
            location_areas: self.location_areas.clone().into_model(self),
            moves: self.moves.clone().into_model(self),
            move_ailments: self.move_ailments.clone().into_model(self),
            move_categories: self.move_categories.clone().into_model(self),
            move_effects: self.move_effects.clone().into_model(self),
            move_flags: self.move_flags.clone().into_model(self),
            move_methods: self.move_methods.clone().into_model(self),
            move_targets: self.move_targets.clone().into_model(self),
            pokedexes: self.pokedexes.clone().into_model(self),
            pokemon: self.pokemon.clone().into_model(self),
            pokemon_forms: self.pokemon_forms.clone().into_model(self),
            pokemon_types: self.pokemon_types.clone().into_model(self),
            pokemon_type_efficacies: self.pokemon_type_efficacies.clone().into_model(self),
            pokemon_type_efficacies_past: self
                .pokemon_type_efficacies_past
                .clone()
                .into_model(self),
            regions: self.regions.clone().into_model(self),
            shapes: self.shapes.clone().into_model(self),
            species: self.species.clone().into_model(self),
            version_groups: self.version_groups.clone().into_model(self),
            versions: self.versions.clone().into_model(self),
            super_contest_effects: self.super_contest_effects.clone().into_model(self),
        }
    }

    fn parse_berry_flavors(&self) -> HashMap<BerryFlavorId, BerryFlavor> {
        self.contest_type_names
            .iter()
            .map(|(id, name_data)| {
                let localizations = name_data
                    .iter()
                    .map(|data| (data.local_language_id, data.flavor.clone()))
                    .collect();
                let names = LocalizedStrings::new(localizations);
                (*id, BerryFlavor { id: *id, names })
            })
            .collect()
    }
}
