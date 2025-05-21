use crate::models::contest_type::ContestTypeId;
use crate::models::localized_names::LocalizedStrings;
use serde::{Deserialize, Serialize};

pub type BerryFlavorId = ContestTypeId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BerryFlavor {
    pub id: BerryFlavorId,
    pub names: LocalizedStrings,
}
