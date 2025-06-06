use crate::models::localized_name_descriptions::LocalizedNameDescriptions;
use serde::{Deserialize, Serialize};

pub type ShapeId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shape {
    pub id: ShapeId,
    pub identifier: String,
    pub prose: LocalizedNameDescriptions,
}
