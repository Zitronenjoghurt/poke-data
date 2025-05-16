use flate2::write::ZlibEncoder;
use flate2::Compression;
use poke_data_parse::RawData;
use std::io::Write;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_path = PathBuf::from("../data");
    let raw_data = RawData::load(&base_path).await?;
    let parsed_data = raw_data.parse();
    let encoded_data = bincode::serialize(&parsed_data).unwrap();

    let output_path = PathBuf::from("../output.bin");
    let mut compressor = ZlibEncoder::new(Vec::new(), Compression::best());
    compressor.write_all(&encoded_data).unwrap();
    let compressed_data = compressor.finish().unwrap();
    std::fs::write(output_path, compressed_data)?;
    Ok(())
}
