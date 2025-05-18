use crate::models::generation::{Generation, GenerationId};
use crate::models::move_method::MoveMethodId;
use crate::models::region::{Region, RegionId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub type VersionGroupId = u8;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UnlinkedVersionGroup {
    pub id: VersionGroupId,
    pub identifier: String,
    pub generation_id: GenerationId,
    pub region_ids: Vec<RegionId>,
    pub move_method_ids: Vec<MoveMethodId>,
    pub order: u8,
}

impl UnlinkedVersionGroup {
    pub fn link(
        &self,
        generations: &HashMap<GenerationId, Arc<Generation>>,
        regions: &HashMap<RegionId, Arc<Region>>,
    ) -> Arc<VersionGroup> {
        let version_group = VersionGroup {
            id: self.id,
            identifier: self.identifier.clone(),
            generation: generations
                .get(&self.generation_id)
                .unwrap_or_else(|| {
                    panic!(
                        "No generation '{}' found for version group '{}'",
                        self.generation_id, self.id
                    )
                })
                .clone(),
            regions: self
                .region_ids
                .iter()
                .map(|region_id| {
                    regions
                        .get(region_id)
                        .unwrap_or_else(|| {
                            panic!(
                                "No region '{}' found for version group '{}'",
                                region_id, self.id
                            )
                        })
                        .clone()
                })
                .collect(),
            move_method_ids: self.move_method_ids.clone(),
            order: self.order,
        };
        Arc::new(version_group)
    }
}

#[derive(Debug)]
pub struct VersionGroup {
    pub id: VersionGroupId,
    pub identifier: String,
    pub generation: Arc<Generation>,
    pub regions: Vec<Arc<Region>>,
    pub move_method_ids: Vec<MoveMethodId>,
    pub order: u8,
}
