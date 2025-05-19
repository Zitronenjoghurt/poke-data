use crate::data::PokeData;
use crate::types::language::Language;
use std::path::PathBuf;

fn load_data() -> PokeData {
    let data_path = PathBuf::from("./data.bin");
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

#[test]
fn test_item() {
    let data = load_data();
    let toxic_orb = data.items.get(&249).unwrap();
    assert_eq!(toxic_orb.identifier, "toxic-orb");
    assert_eq!(
        toxic_orb.names.get_by_language(Language::English),
        "Toxic Orb"
    );
    assert_eq!(toxic_orb.cost, 4000);
    assert_eq!(toxic_orb.fling_power, Some(30));
    assert_eq!(
        toxic_orb.category.names.get_by_language(Language::English),
        "Bad held items"
    );
    assert_eq!(toxic_orb.category.pocket.identifier, "misc");
    assert_eq!(
        toxic_orb
            .fling_effect
            .clone()
            .unwrap()
            .effects
            .get_by_language(Language::English)
            .unwrap()
            .effect,
        "Badly poisons the target."
    );
    assert_eq!(toxic_orb.flags.len(), 2);
}

#[test]
fn test_damage_class() {
    let data = load_data();
    let physical = data.damage_classes.get(&2).unwrap();
    assert_eq!(physical.identifier, "physical");
    assert_eq!(
        physical
            .prose
            .get_by_language(Language::English)
            .unwrap()
            .description,
        "Physical damage, controlled by Attack and Defense"
    );
}
