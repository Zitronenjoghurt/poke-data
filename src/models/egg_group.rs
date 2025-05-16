use crate::models::localized_names::LocalizedNames;
use serde::{Deserialize, Serialize};

pub type EggGroupId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EggGroup {
    pub id: EggGroupId,
    pub identifier: String,
    pub names: LocalizedNames,
}
