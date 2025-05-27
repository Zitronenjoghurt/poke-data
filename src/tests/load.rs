use crate::data_structures::entity_collection::HasNameSearchIndex;
use crate::tests::load_data;
use crate::types::language::Language;
use crate::types::pokemon_type::Type;
use crate::types::stat::Stat;

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
    assert_eq!(cheri.flavors.len(), 1);

    let (flavor, flavor_amount) = &cheri.flavors[0];
    assert_eq!(flavor.names.get_by_language(Language::English), "Spicy");
    assert_eq!(*flavor_amount, 10);
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
fn test_move() {
    let data = load_data();
    let hyper_beam = data.moves.find_by_name("hypa beamm", 0.5).unwrap();
    assert_eq!(hyper_beam.identifier, "hyper-beam");
    assert_eq!(
        hyper_beam.names.get_by_language(Language::French),
        "Ultralaser"
    );
    assert_eq!(hyper_beam.power, Some(150));
    assert_eq!(hyper_beam.accuracy, Some(90));
    assert_eq!(hyper_beam.pp, Some(5));
    assert_eq!(hyper_beam.generation.identifier, "generation-i");
    assert_eq!(hyper_beam.pokemon_type, Type::Normal);
    assert_eq!(hyper_beam.target.identifier, "selected-pokemon");
    assert_eq!(hyper_beam.damage_class.identifier, "special");

    let contest_type = hyper_beam.contest_type.clone().unwrap();
    assert_eq!(contest_type.identifier, "cool");
}

#[test]
fn test_pokedex() {
    let data = load_data();
    let national = data.pokedexes.find_by_name("national", 0.5).unwrap();

    let bulbasaur = national.species_map.get(&1).unwrap();
    assert_eq!(bulbasaur.identifier, "bulbasaur");
}

#[test]
fn test_pokemon() {
    let data = load_data();
    let mareep = data.pokemon.find_by_name("mareeep", 0.5).unwrap();
    assert_eq!(mareep.identifier, "mareep");
    assert_eq!(mareep.base_stats.get_value(Stat::Hp), 55);
    assert_eq!(mareep.base_stats.get_value(Stat::Attack), 40);
    assert_eq!(mareep.base_stats.get_value(Stat::Defense), 40);
    assert_eq!(mareep.base_stats.get_value(Stat::SpAttack), 65);
    assert_eq!(mareep.base_stats.get_value(Stat::SpDefense), 45);
    assert_eq!(mareep.base_stats.get_value(Stat::Speed), 35);
    assert_eq!(mareep.get_types(1).clone(), vec![Type::Electric]);

    let giratina = data.pokemon.find_by_name("giratina", 0.5).unwrap();
    assert_eq!(
        giratina.get_types(1).clone(),
        vec![Type::Ghost, Type::Dragon]
    );

    let magnemite = data.pokemon.find_by_name("magnemite", 0.5).unwrap();
    assert_eq!(magnemite.get_types(1).clone(), vec![Type::Electric]);
    assert_eq!(
        magnemite.get_types(2).clone(),
        vec![Type::Electric, Type::Steel]
    );

    let clefairy = data.pokemon.find_by_name("clefairy", 0.5).unwrap();
    assert_eq!(clefairy.get_types(1).clone(), vec![Type::Normal]);
    assert_eq!(clefairy.get_types(2).clone(), vec![Type::Normal]);
    assert_eq!(clefairy.get_types(3).clone(), vec![Type::Normal]);
    assert_eq!(clefairy.get_types(4).clone(), vec![Type::Normal]);
    assert_eq!(clefairy.get_types(5).clone(), vec![Type::Normal]);
    assert_eq!(clefairy.get_types(6).clone(), vec![Type::Fairy]);
    assert_eq!(clefairy.get_types(7).clone(), vec![Type::Fairy]);
    assert_eq!(clefairy.get_types(8).clone(), vec![Type::Fairy]);
    assert_eq!(clefairy.get_types(9).clone(), vec![Type::Fairy]);
    assert_eq!(clefairy.get_types(10).clone(), vec![Type::Fairy]);

    let bulbasaur = data.pokemon.find_by_name("bulbasaure", 0.5).unwrap();
    assert_eq!(bulbasaur.identifier, "bulbasaur");
    assert_eq!(bulbasaur.get_abilities(1).len(), 1);
    assert_eq!(bulbasaur.get_abilities(2).len(), 1);
    assert_eq!(bulbasaur.get_abilities(3).len(), 1);
    assert_eq!(bulbasaur.get_abilities(4).len(), 1);
    assert_eq!(bulbasaur.get_abilities(5).len(), 2);

    let overgrow = &bulbasaur.get_abilities(1)[0];
    assert_eq!(overgrow.ability.identifier, "overgrow");

    let chlorophyll = &bulbasaur
        .get_abilities(5)
        .into_iter()
        .find(|ability| ability.is_hidden)
        .unwrap();
    assert_eq!(chlorophyll.ability.identifier, "chlorophyll");
}

#[test]
fn test_species() {
    let data = load_data();
    let snorlax = data.species.find_by_name("snorlex", 0.5).unwrap();
    assert_eq!(snorlax.identifier, "snorlax");
    assert_eq!(snorlax.generation.identifier, "generation-i");
    assert_eq!(snorlax.growth_rate.identifier, "slow");

    let pikachu = data.species.find_by_name("pikachu", 0.5).unwrap();
    assert_eq!(pikachu.identifier, "pikachu");
    let evolution = &pikachu.evolutions[0];
    assert_eq!(evolution.evolves_into_species_id, 26);
    assert_eq!(evolution.trigger.identifier, "use-item");
    assert_eq!(
        evolution.trigger_item.clone().unwrap().identifier,
        "thunder-stone"
    );
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
