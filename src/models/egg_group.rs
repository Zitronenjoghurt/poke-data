use crate::models::localized_names::LocalizedStrings;
use crate::traits::has_identifier::HasIdentifier;
use crate::traits::has_localized_names::HasLocalizedNames;
use serde::{Deserialize, Serialize};

pub type EggGroupId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EggGroup {
    pub id: EggGroupId,
    pub identifier: String,
    pub names: LocalizedStrings,
}

impl HasLocalizedNames for EggGroup {
    fn localized_names(&self) -> &LocalizedStrings {
        &self.names
    }
}

impl HasIdentifier for EggGroup {
    fn identifier(&self) -> &str {
        &self.identifier
    }
}
