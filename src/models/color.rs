use crate::models::localized_names::LocalizedNames;
use serde::{Deserialize, Serialize};

pub type ColorId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub id: ColorId,
    pub identifier: String,
    pub names: LocalizedNames,
}
