use crate::models::localized_names::LocalizedStrings;
use serde::{Deserialize, Serialize};

pub type ContestTypeId = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContestType {
    pub id: ContestTypeId,
    pub identifier: String,
    pub names: LocalizedStrings,
}
