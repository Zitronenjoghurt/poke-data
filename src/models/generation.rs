use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::localized_names::LocalizedStrings;
use crate::models::region::{Region, RegionId};
use crate::traits::has_identifier::HasIdentifier;
use crate::traits::has_localized_names::HasLocalizedNames;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type GenerationId = u16;

#[derive(Debug)]
pub struct Generation {
    pub id: GenerationId,
    pub identifier: String,
    pub main_region: Arc<Region>,
    pub names: LocalizedStrings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedGeneration {
    pub id: GenerationId,
    pub identifier: String,
    pub main_region_id: RegionId,
    pub names: LocalizedStrings,
}

impl Linkable for UnlinkedGeneration {
    type Linked = Arc<Generation>;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let main_region = context
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

impl HasLocalizedNames for Generation {
    fn localized_names(&self) -> &LocalizedStrings {
        &self.names
    }
}

impl HasIdentifier for Generation {
    fn identifier(&self) -> &str {
        &self.identifier
    }
}
