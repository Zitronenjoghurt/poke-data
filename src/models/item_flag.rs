use crate::models::localized_name_descriptions::LocalizedNameDescriptions;
use serde::{Deserialize, Serialize};

pub type ItemFlagId = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemFlag {
    pub id: ItemFlagId,
    pub identifier: String,
    pub prose: LocalizedNameDescriptions,
}
