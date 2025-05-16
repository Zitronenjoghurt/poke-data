use crate::PokeData;
use std::path::PathBuf;

#[test]
fn test_load() {
    let data_path = PathBuf::from("./output.bin");
    let data = PokeData::load(&data_path).unwrap();

    let stench = data.abilities.get(&1).unwrap();
    assert_eq!(stench.identifier, "stench");
    assert_eq!(stench.generation.identifier, "generation-iii");
    assert_eq!(stench.generation.main_region.identifier, "hoenn");

    let pokemon = data.pokemon.get(&1).unwrap();
    println!("{:#?}", pokemon);
}
