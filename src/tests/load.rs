use crate::tests::load_data;
use crate::types::language::Language;

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
fn test_berry() {
    let data = load_data();
    let cheri = data.berries.get(&1).unwrap();
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
fn test_color() {
    let data = load_data();
    let green = data.colors.get(&5).unwrap();
    assert_eq!(green.identifier, "green");
    assert_eq!(green.names.get_by_language(Language::English), "Green");
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

#[test]
fn test_egg_groups() {
    let data = load_data();
    let dragon = data.egg_groups.get(&14).unwrap();
    assert_eq!(dragon.identifier, "dragon");
    assert_eq!(dragon.names.get_by_language(Language::English), "Dragon");
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
fn test_pokemon() {
    let data = load_data();
    let mareep = data.pokemon.get(&179).unwrap();
    assert_eq!(mareep.identifier, "mareep");
    println!("{:#?}", mareep);
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
fn test_types() {
    let data = load_data();
    let electric = data.pokemon_types.get(&13).unwrap();
    assert_eq!(electric.identifier, "electric");
    assert_eq!(electric.generation.id, 1);
    assert_eq!(electric.damage_class.clone().unwrap().identifier, "special");
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
