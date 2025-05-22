use crate::models::RemoteModel;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::{Path, PathBuf};

pub mod ability;
pub mod ability_changelog;
pub mod ability_changelog_prose;
pub mod ability_flavor_text;
pub mod ability_names;
pub mod ability_prose;
pub mod berries;
pub mod berry_firmness;
pub mod berry_firmness_names;
pub mod berry_flavors;
pub mod contest_effect_prose;
pub mod contest_effects;
pub mod contest_type_names;
pub mod contest_types;
pub mod egg_group;
pub mod egg_group_prose;
pub mod encounter;
pub mod encounter_condition_prose;
pub mod encounter_condition_value_map;
pub mod encounter_condition_value_prose;
pub mod encounter_condition_values;
pub mod encounter_conditions;
pub mod encounter_method_prose;
pub mod encounter_methods;
pub mod encounter_slots;
pub mod evolution_chains;
pub mod evolution_trigger_prose;
pub mod evolution_triggers;
pub mod experience;
pub mod generation;
pub mod generation_names;
pub mod growth_rate_prose;
pub mod growth_rates;
pub mod item_categories;
pub mod item_category_prose;
pub mod item_flag_map;
pub mod item_flag_prose;
pub mod item_flags;
pub mod item_flavor_text;
pub mod item_fling_effect_prose;
pub mod item_fling_effects;
pub mod item_game_indices;
pub mod item_names;
pub mod item_pocket_names;
pub mod item_pockets;
pub mod item_prose;
pub mod items;
pub mod location_area_encounter_rates;
pub mod location_area_prose;
pub mod location_areas;
pub mod location_game_indices;
pub mod location_names;
pub mod locations;
pub mod move_ailment_names;
pub mod move_ailments;
pub mod move_categories;
pub mod move_category_prose;
pub mod move_changelogs;
pub mod move_damage_class_prose;
pub mod move_damage_classes;
pub mod move_effect_changelog;
pub mod move_effect_changelog_prose;
pub mod move_effect_prose;
pub mod move_effects;
pub mod move_flag_map;
pub mod move_flag_prose;
pub mod move_flags;
pub mod move_flavor_texts;
pub mod move_meta;
pub mod move_meta_stat_changes;
pub mod move_method_prose;
pub mod move_methods;
pub mod move_names;
pub mod move_target_prose;
pub mod move_targets;
pub mod moves;
pub mod pokemon;
pub mod pokemon_abilities;
pub mod pokemon_color_names;
pub mod pokemon_colors;
pub mod pokemon_habitat_names;
pub mod pokemon_habitats;
pub mod pokemon_move_map;
pub mod pokemon_shape_prose;
pub mod pokemon_shapes;
pub mod pokemon_species;
pub mod pokemon_species_names;
pub mod pokemon_stats;
pub mod pokemon_type_efficacy;
pub mod pokemon_type_efficacy_past;
pub mod pokemon_type_game_indices;
pub mod pokemon_type_names;
pub mod pokemon_types;
pub mod pokemon_types_map;
pub mod pokemon_types_past_map;
pub mod region;
pub mod region_names;
pub mod super_contest_effect_prose;
pub mod super_contest_effects;
pub mod version_group_pokemon_move_methods;
pub mod version_group_regions;
pub mod version_groups;
pub mod version_names;
pub mod versions;

pub const POKEAPI_DATA_BASE_URL: &str =
    "https://raw.githubusercontent.com/PokeAPI/pokeapi/refs/heads/master/data/v2/csv";

pub trait PokeApiModel: RemoteModel + Serialize + for<'a> Deserialize<'a> {
    fn file_name() -> &'static str;
}

impl<T: PokeApiModel> RemoteModel for T {
    fn source_url() -> String {
        let file_name = Self::file_name();
        format!("{POKEAPI_DATA_BASE_URL}/{file_name}.csv")
    }

    fn source_file_path(base_path: &Path) -> PathBuf {
        let file_name = Self::file_name();
        base_path.join(format!("{file_name}.csv"))
    }

    fn parse(content: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        csv::Reader::from_reader(content.as_bytes())
            .deserialize()
            .collect::<Result<Vec<Self>, csv::Error>>()
            .map_err(|e| e.into())
    }
}
