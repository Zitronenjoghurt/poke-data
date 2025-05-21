use crate::models::localized_names::LocalizedStrings;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type GrowthRateId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthRate {
    pub id: GrowthRateId,
    pub identifier: String,
    pub formula: String,
    pub names: LocalizedStrings,
    pub experience: GrowthRateExperience,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Total experience points needed to reach each level.
pub struct GrowthRateExperience(HashMap<u8, u32>);

impl GrowthRateExperience {
    pub fn new(experiences: HashMap<u8, u32>) -> Self {
        Self(experiences)
    }
}
