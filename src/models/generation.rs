use crate::data::linkable::Linkable;
use crate::data::PokeData;
use crate::models::localized_names::LocalizedNames;
use crate::models::region::{Region, RegionId};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type GenerationId = u16;

#[derive(Debug)]
pub struct Generation {
    pub id: GenerationId,
    pub identifier: String,
    pub main_region: Arc<Region>,
    pub names: LocalizedNames,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedGeneration {
    pub id: GenerationId,
    pub identifier: String,
    pub main_region_id: RegionId,
    pub names: LocalizedNames,
}

impl Linkable for UnlinkedGeneration {
    type Linked = Arc<Generation>;

    fn link(&self, data: &PokeData) -> Self::Linked {
        let main_region = data
            .regions
            .get(&self.main_region_id)
            .unwrap_or_else(|| {
                panic!(
                    "No region '{}' found for generation '{}'",
                    self.main_region_id, self.id
                )
            })
            .clone();

        let generation = Generation {
            id: self.id,
            identifier: self.identifier.clone(),
            main_region,
            names: self.names.clone(),
        };

        Arc::new(generation)
    }
}
