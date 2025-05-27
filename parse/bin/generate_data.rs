use poke_data_parse::raw_assets::load_assets;
use poke_data_parse::raw_data::RawData;
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_path = PathBuf::from("../data");
    build_data(&base_path).await?;
    build_assets(&base_path).await?;
    Ok(())
}

async fn build_data(base_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let raw_data = RawData::load(base_path).await?;
    let parsed_data = raw_data.parse();
    let encoded_data = bincode::serialize(&parsed_data)?;
    let output_path = PathBuf::from("../data.bin");
    let compressed_data = zstd::encode_all(encoded_data.as_slice(), 22)?;
    std::fs::write(output_path, compressed_data)?;
    Ok(())
}

async fn build_assets(base_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let assets = load_assets(base_path)?;
    let encoded_data = bincode::serialize(&assets)?;
    let output_path = PathBuf::from("../assets.bin");
    let compressed_data = zstd::encode_all(encoded_data.as_slice(), 22)?;
    std::fs::write(output_path, compressed_data)?;
    Ok(())
}
