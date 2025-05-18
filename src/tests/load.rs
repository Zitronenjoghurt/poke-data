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
    println!("{:#?}", snorlax);
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
