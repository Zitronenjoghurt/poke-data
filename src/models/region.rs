use crate::models::localized_names::LocalizedNames;
use crate::traits::has_identifier::HasIdentifier;
use crate::traits::has_localized_names::HasLocalizedNames;
use serde::{Deserialize, Serialize};

pub type RegionId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub id: RegionId,
    pub identifier: String,
    pub names: LocalizedNames,
}

impl HasIdentifier for Region {
    fn identifier(&self) -> &str {
        &self.identifier
    }
}

impl HasLocalizedNames for Region {
    fn localized_names(&self) -> &LocalizedNames {
        &self.names
    }
}
