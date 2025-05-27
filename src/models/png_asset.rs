use image::DynamicImage;
use oxipng::{optimize_from_memory, Options, StripChunks};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PngAsset(Vec<u8>);

impl PngAsset {
    pub fn new(data: &[u8]) -> Result<Self, Box<dyn Error>> {
        let options = Options {
            strip: StripChunks::Safe,
            ..Default::default()
        };
        Ok(Self(optimize_from_memory(data, &options)?))
    }

    pub fn load(&self) -> Result<DynamicImage, Box<dyn Error>> {
        Ok(image::load_from_memory(&self.0)?)
    }
}
