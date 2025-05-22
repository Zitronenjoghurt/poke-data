use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::localized_name_descriptions::LocalizedNameDescriptions;
use crate::models::region::{Region, RegionId};
use crate::models::species::{Species, SpeciesId};
use crate::models::version_group::{VersionGroup, VersionGroupId};
use crate::traits::has_identifier::HasIdentifier;
use crate::traits::has_localized_name_descriptions::HasLocalizedNameDescriptions;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;

pub type PokedexId = u8;

#[derive(Debug)]
pub struct Pokedex {
    pub id: PokedexId,
    pub identifier: String,
    pub prose: LocalizedNameDescriptions,
    pub region: Option<Arc<Region>>,
    pub is_main_series: bool,
    pub version_groups: Vec<Arc<VersionGroup>>,
    pub species_map: BTreeMap<u16, Arc<Species>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedPokedex {
    pub id: PokedexId,
    pub identifier: String,
    pub prose: LocalizedNameDescriptions,
    pub region_id: Option<RegionId>,
    pub is_main_series: bool,
    pub version_group_ids: Vec<VersionGroupId>,
    pub species_map: BTreeMap<u16, SpeciesId>,
}

impl Linkable for UnlinkedPokedex {
    type Linked = Arc<Pokedex>;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let region = self.region_id.map(|region_id| {
            context
                .regions
                .get(&region_id)
                .unwrap_or_else(|| {
                    panic!("No region '{}' found for pokedex '{}'", region_id, self.id)
                })
                .clone()
        });

        let version_groups = self
            .version_group_ids
            .iter()
            .map(|version_group_id| {
                context
                    .version_groups
                    .get(version_group_id)
                    .unwrap_or_else(|| {
                        panic!(
                            "No version group '{}' found for pokedex '{}'",
                            version_group_id, self.id
                        )
                    })
                    .clone()
            })
            .collect();

        let species_map = self
            .species_map
            .iter()
            .map(|(id, species_id)| {
                (
                    *id,
                    context
                        .species
                        .get(species_id)
                        .unwrap_or_else(|| {
                            panic!(
                                "No species '{}' found for pokedex '{}'",
                                species_id, self.id
                            )
                        })
                        .clone(),
                )
            })
            .collect();

        let pokedex = Pokedex {
            id: self.id,
            identifier: self.identifier.clone(),
            prose: self.prose.clone(),
            region,
            is_main_series: false,
            version_groups,
            species_map,
        };

        Arc::new(pokedex)
    }
}

impl HasIdentifier for Pokedex {
    fn identifier(&self) -> &str {
        &self.identifier
    }
}

impl HasLocalizedNameDescriptions for Pokedex {
    fn localized_name_descriptions(&self) -> &LocalizedNameDescriptions {
        &self.prose
    }
}
