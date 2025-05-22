use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::generation::{Generation, GenerationId};
use crate::models::pokemon_move_method::PokemonMoveMethodId;
use crate::models::region::{Region, RegionId};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type VersionGroupId = u8;

#[derive(Debug)]
pub struct VersionGroup {
    pub id: VersionGroupId,
    pub identifier: String,
    pub generation: Arc<Generation>,
    pub regions: Vec<Arc<Region>>,
    pub move_method_ids: Vec<PokemonMoveMethodId>,
    pub order: u8,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UnlinkedVersionGroup {
    pub id: VersionGroupId,
    pub identifier: String,
    pub generation_id: GenerationId,
    pub region_ids: Vec<RegionId>,
    pub move_method_ids: Vec<PokemonMoveMethodId>,
    pub order: u8,
}

impl Linkable for UnlinkedVersionGroup {
    type Linked = Arc<VersionGroup>;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let generation = context
            .generations
            .get(&self.generation_id)
            .unwrap_or_else(|| {
                panic!(
                    "No generation '{}' found for version group '{}'",
                    self.generation_id, self.id
                )
            })
            .clone();

        let regions = self
            .region_ids
            .iter()
            .map(|region_id| {
                context
                    .regions
                    .get(region_id)
                    .unwrap_or_else(|| {
                        panic!(
                            "No region '{}' found for version group '{}'",
                            region_id, self.id
                        )
                    })
                    .clone()
            })
            .collect();

        let version_group = VersionGroup {
            id: self.id,
            identifier: self.identifier.clone(),
            generation,
            regions,
            move_method_ids: self.move_method_ids.clone(),
            order: self.order,
        };

        Arc::new(version_group)
    }
}
