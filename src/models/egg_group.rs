use crate::models::localized_names::LocalizedNames;
use crate::traits::has_identifier::HasIdentifier;
use crate::traits::has_localized_names::HasLocalizedNames;
use serde::{Deserialize, Serialize};

pub type EggGroupId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EggGroup {
    pub id: EggGroupId,
    pub identifier: String,
    pub names: LocalizedNames,
}

impl HasLocalizedNames for EggGroup {
    fn localized_names(&self) -> &LocalizedNames {
        &self.names
    }
}

impl HasIdentifier for EggGroup {
    fn identifier(&self) -> &str {
        &self.identifier
    }
}
