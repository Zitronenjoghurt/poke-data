use crate::models::png_asset::PngAsset;
use crate::models::pokemon::PokemonId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct PokemonAssets {
    pub official_artworks: HashMap<PokemonId, PngAsset>,
}

impl PokemonAssets {
    pub fn load_from_path(compressed_data_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let data = std::fs::read(compressed_data_path)?;
        let decompressed_data = zstd::decode_all(data.as_slice())?;
        Ok(bincode::deserialize(&decompressed_data)?)
    }

    pub fn load_from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(bincode::deserialize(bytes)?)
    }
}
