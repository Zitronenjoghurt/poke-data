use crate::models::localized_names::LocalizedNames;
use serde::{Deserialize, Serialize};

pub type RegionId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub id: RegionId,
    pub identifier: String,
    pub names: LocalizedNames,
}
