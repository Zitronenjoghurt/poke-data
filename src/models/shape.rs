use crate::models::language::LanguageId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ShapeId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shape {
    pub id: ShapeId,
    pub identifier: String,
    pub prose: ShapeProse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeProse(HashMap<LanguageId, ShapeProseEntry>);

impl ShapeProse {
    pub fn new(entries: HashMap<LanguageId, ShapeProseEntry>) -> Self {
        Self(entries)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeProseEntry {
    pub name: String,
    pub awesome_name: String,
    pub description: String,
}
