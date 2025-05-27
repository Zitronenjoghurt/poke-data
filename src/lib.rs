pub mod assets;
pub mod collections;
pub mod data;
mod data_structures;
pub mod models;
#[cfg(test)]
mod tests;
mod traits;
pub mod types;

pub const INCLUDED_DATA: &[u8] = include_bytes!("../data.bin");
