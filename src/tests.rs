use crate::data::PokeData;
use std::path::PathBuf;

mod load;
mod type_efficacy;

fn load_data() -> PokeData {
    let data_path = PathBuf::from("./data.bin");
    PokeData::load_path(&data_path).unwrap()
}
