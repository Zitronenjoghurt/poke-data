use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct GenderRate(i8);

impl GenderRate {
    pub fn new(value: i8) -> Self {
        Self(value)
    }

    pub fn value(&self) -> i8 {
        self.0
    }
}
