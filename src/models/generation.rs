use crate::models::localized_names::LocalizedNames;
use crate::models::region::{Region, RegionId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub type GenerationId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedGeneration {
    pub id: GenerationId,
    pub identifier: String,
    pub main_region_id: RegionId,
    pub names: LocalizedNames,
}

impl UnlinkedGeneration {
    pub fn link(&self, regions: &HashMap<RegionId, Arc<Region>>) -> Arc<Generation> {
        let generation = Generation {
            id: self.id,
            identifier: self.identifier.clone(),
            main_region: regions.get(&self.main_region_id).unwrap().clone(),
            names: self.names.clone(),
        };
        Arc::new(generation)
    }
}

#[derive(Debug)]
pub struct Generation {
    pub id: GenerationId,
    pub identifier: String,
    pub main_region: Arc<Region>,
    pub names: LocalizedNames,
}
