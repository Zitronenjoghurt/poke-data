use crate::models::RemoteModel;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::{Path, PathBuf};

pub mod ability;
pub mod ability_changelog;
pub mod ability_changelog_prose;
pub mod ability_flavor_text;
pub mod ability_names;
pub mod ability_prose;
pub mod egg_group;
pub mod egg_group_prose;
pub mod generation;
pub mod generation_names;
pub mod pokemon;
pub mod pokemon_abilities;
pub mod region;
pub mod region_names;

pub const POKEAPI_DATA_BASE_URL: &str =
    "https://raw.githubusercontent.com/PokeAPI/pokeapi/refs/heads/master/data/v2/csv";

pub trait PokeApiModel: RemoteModel + Serialize + for<'a> Deserialize<'a> {
    fn file_name() -> &'static str;
}

impl<T: PokeApiModel> RemoteModel for T {
    fn source_url() -> String {
        let file_name = Self::file_name();
        format!("{POKEAPI_DATA_BASE_URL}/{file_name}.csv")
    }

    fn source_file_path(base_path: &Path) -> PathBuf {
        let file_name = Self::file_name();
        base_path.join(format!("{file_name}.csv"))
    }

    fn parse(content: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        csv::Reader::from_reader(content.as_bytes())
            .deserialize()
            .collect::<Result<Vec<Self>, csv::Error>>()
            .map_err(|e| e.into())
    }
}
