use crate::models::generation::GenerationId;
use crate::models::language::LanguageId;
use crate::models::region::{Region, RegionId};
use crate::types::language::Language;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub type LocationId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedLocation {
    pub id: LocationId,
    pub identifier: String,
    pub region_id: Option<RegionId>,
    pub names: LocalizedLocationNames,
    pub game_indices: LocationGameIndices,
}

impl UnlinkedLocation {
    pub fn link(&self, regions: &HashMap<RegionId, Arc<Region>>) -> Arc<Location> {
        let location = Location {
            id: self.id,
            identifier: self.identifier.clone(),
            region: self.region_id.map(|region_id| {
                regions
                    .get(&region_id)
                    .unwrap_or_else(|| {
                        panic!("No region '{}' found for location '{}'", region_id, self.id)
                    })
                    .clone()
            }),
            names: self.names.clone(),
            game_indices: self.game_indices.clone(),
        };
        Arc::new(location)
    }
}

#[derive(Debug)]
pub struct Location {
    pub id: LocationId,
    pub identifier: String,
    pub region: Option<Arc<Region>>,
    pub names: LocalizedLocationNames,
    pub game_indices: LocationGameIndices,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationGameIndices(HashMap<GenerationId, u16>);

impl LocationGameIndices {
    pub fn new(indices: HashMap<GenerationId, u16>) -> Self {
        Self(indices)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedLocationNames(HashMap<LanguageId, LocalizedLocationName>);

impl LocalizedLocationNames {
    pub fn new(localizations: HashMap<LanguageId, LocalizedLocationName>) -> Self {
        Self(localizations)
    }

    pub fn get_by_language(&self, language: Language) -> Option<&LocalizedLocationName> {
        let language_id = language as LanguageId;
        if let Some(target) = self.0.get(&language_id) {
            return Some(target);
        }

        let default_language_id = Language::default() as LanguageId;
        if let Some(default) = self.0.get(&default_language_id) {
            return Some(default);
        }

        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedLocationName {
    pub name: String,
    pub subtitle: String,
}
