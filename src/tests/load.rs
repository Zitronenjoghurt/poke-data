use crate::data::PokeData;
use crate::types::language::Language;
use std::path::PathBuf;

fn load_data() -> PokeData {
    let data_path = PathBuf::from("./output.bin");
    PokeData::load_path(&data_path).unwrap()
}

#[test]
fn test_ability() {
    let data = load_data();
    let stench = data.abilities.get(&1).unwrap();
    assert_eq!(stench.identifier, "stench");
    assert_eq!(stench.names.get_by_language(Language::English), "Stench");
    assert_eq!(stench.generation.identifier, "generation-iii");
    assert_eq!(stench.generation.main_region.identifier, "hoenn");
}

#[test]
fn test_species() {
    let data = load_data();
    let snorlax = data.species.get(&143).unwrap();
    assert_eq!(snorlax.identifier, "snorlax");
    assert_eq!(snorlax.generation.identifier, "generation-i");
    assert_eq!(snorlax.growth_rate.identifier, "slow");
}

#[test]
fn test_version() {
    let data = load_data();
    let red = data.versions.get(&1).unwrap();
    assert_eq!(red.identifier, "red");
    assert_eq!(red.names.get_by_language(Language::German), "Rot");
    assert_eq!(red.version_group.identifier, "red-blue");
    assert_eq!(red.version_group.generation.identifier, "generation-i");
    assert_eq!(red.version_group.regions[0].identifier, "kanto")
}

#[test]
fn test_location() {
    let data = load_data();
    let area = data.location_areas.get(&211).unwrap();
    assert_eq!(area.game_index, 28);
    assert_eq!(
        area.location
            .names
            .get_by_language(Language::English)
            .unwrap()
            .name,
        "Ecruteak City"
    );
}

#[test]
fn test_encounter_method() {
    let data = load_data();
    let method = data.encounter_methods.get(&1).unwrap();
    assert_eq!(method.identifier, "walk");
    assert_eq!(
        method.names.get_by_language(Language::English),
        "Walking in tall grass or a cave"
    );
}
