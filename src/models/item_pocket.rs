use crate::models::localized_names::LocalizedStrings;
use serde::{Deserialize, Serialize};

pub type ItemPocketId = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPocket {
    pub id: ItemPocketId,
    pub identifier: String,
    pub names: LocalizedStrings,
}
