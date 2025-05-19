use crate::tests::load_data;
use crate::types::pokemon_type::Type;

#[test]
fn test_type_efficacy() {
    let data = load_data();
    assert_eq!(
        data.pokemon_type_efficacies.get_damage_factor(
            10,
            Type::Rock,
            vec![Type::Fighting, Type::Ground],
        ),
        0.25
    );

    assert_eq!(
        data.pokemon_type_efficacies
            .get_damage_factor(1, Type::Ghost, vec![Type::Psychic]),
        0.0
    );

    assert_eq!(
        data.pokemon_type_efficacies
            .get_damage_factor(2, Type::Ghost, vec![Type::Psychic]),
        2.0
    );

    assert_eq!(
        data.pokemon_type_efficacies
            .get_damage_factor(6, Type::Ghost, vec![Type::Psychic]),
        2.0
    );

    assert_eq!(
        data.pokemon_type_efficacies
            .get_damage_factor(10, Type::Ghost, vec![Type::Psychic]),
        2.0
    );

    assert_eq!(
        data.pokemon_type_efficacies
            .get_damage_factor(1, Type::Ghost, vec![Type::Steel]),
        0.5
    );

    assert_eq!(
        data.pokemon_type_efficacies
            .get_damage_factor(5, Type::Ghost, vec![Type::Steel]),
        0.5
    );

    assert_eq!(
        data.pokemon_type_efficacies
            .get_damage_factor(6, Type::Ghost, vec![Type::Steel]),
        1.0
    );

    assert_eq!(
        data.pokemon_type_efficacies
            .get_damage_factor(10, Type::Ghost, vec![Type::Steel]),
        1.0
    );
}
