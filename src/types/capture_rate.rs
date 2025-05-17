use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CaptureRate(u8);

impl CaptureRate {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}
