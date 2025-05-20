use crate::data_structures::entity_collection::HasNameSearchIndex;
use crate::tests::load_data;
use crate::types::language::Language;

#[test]
fn test_ability() {
    let data = load_data();
    let stench = data.abilities.find_by_name("strench", 0.5).unwrap();
    assert_eq!(stench.identifier, "stench");
    assert_eq!(stench.names.get_by_language(Language::English), "Stench");
    assert_eq!(stench.generation.identifier, "generation-iii");
    assert_eq!(stench.generation.main_region.identifier, "hoenn");
}

#[test]
fn test_berry() {
    let data = load_data();
    let cheri = data.berries.find_by_name("chery", 0.5).unwrap();
    assert_eq!(cheri.item.identifier, "cheri-berry");
    assert_eq!(
        cheri.firmness.names.get_by_language(Language::English),
        "Soft"
    );
    assert_eq!(cheri.size, 20);
    assert_eq!(cheri.max_harvest, 5);
    assert_eq!(cheri.growth_time, 3);
    assert_eq!(cheri.soil_dryness, 15);
    assert_eq!(cheri.smoothness, 25);
}

#[test]
fn test_egg_groups() {
    let data = load_data();
    let dragon = data.egg_groups.find_by_name("dragpn", 0.5).unwrap();
    assert_eq!(dragon.identifier, "dragon");
    assert_eq!(dragon.names.get_by_language(Language::English), "Dragon");
}

#[test]
fn test_item() {
    let data = load_data();
    let toxic_orb = data.items.find_by_name("Toxic orb", 0.5).unwrap();
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
fn test_location() {
    let data = load_data();
    let area = data.location_areas.find_by_name("Ekruteak", 0.5).unwrap();
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
fn test_pokemon() {
    let data = load_data();
    let mareep = data.pokemon.find_by_name("mareeep", 0.5).unwrap();
    assert_eq!(mareep.identifier, "mareep");
}

#[test]
fn test_species() {
    let data = load_data();
    let snorlax = data.species.find_by_name("snorlex", 0.5).unwrap();
    assert_eq!(snorlax.identifier, "snorlax");
    assert_eq!(snorlax.generation.identifier, "generation-i");
    assert_eq!(snorlax.growth_rate.identifier, "slow");
}

#[test]
fn test_version() {
    let data = load_data();
    let red = data.versions.find_by_name("redd", 0.5).unwrap();
    assert_eq!(red.identifier, "red");
    assert_eq!(red.names.get_by_language(Language::German), "Rot");
    assert_eq!(red.version_group.identifier, "red-blue");
    assert_eq!(red.version_group.generation.identifier, "generation-i");
    assert_eq!(red.version_group.regions[0].identifier, "kanto")
}
