use crate::data::PokeData;

mod load;
mod type_efficacy;

fn load_data() -> PokeData {
    PokeData::load().unwrap()
}
