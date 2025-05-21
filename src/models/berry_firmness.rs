use crate::models::localized_names::LocalizedStrings;
use serde::{Deserialize, Serialize};

pub type BerryFirmnessId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BerryFirmness {
    pub id: BerryFirmnessId,
    pub identifier: String,
    pub names: LocalizedStrings,
}
