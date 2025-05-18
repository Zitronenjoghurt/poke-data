use crate::models::localized_name_descriptions::LocalizedNameDescriptions;
use serde::{Deserialize, Serialize};

pub type DamageClassId = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageClass {
    pub id: DamageClassId,
    pub identifier: String,
    pub prose: LocalizedNameDescriptions,
}
