use poke_data_parse::RawData;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_path = PathBuf::from("../data");
    let raw_data = RawData::load(&base_path).await?;
    let parsed_data = raw_data.parse();
    let encoded_data = bincode::serialize(&parsed_data)?;

    let output_path = PathBuf::from("../data.bin");
    let compressed_data = zstd::encode_all(encoded_data.as_slice(), 22)?;
    std::fs::write(output_path, compressed_data)?;
    Ok(())
}
