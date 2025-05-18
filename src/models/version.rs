use crate::models::localized_names::LocalizedNames;
use crate::models::version_group::{VersionGroup, VersionGroupId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub type VersionId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedVersion {
    pub id: VersionId,
    pub identifier: String,
    pub names: LocalizedNames,
    pub version_group_id: VersionGroupId,
}

impl UnlinkedVersion {
    pub fn link(
        &self,
        version_groups: &HashMap<VersionGroupId, Arc<VersionGroup>>,
    ) -> Arc<Version> {
        let version = Version {
            id: self.id,
            identifier: self.identifier.clone(),
            names: self.names.clone(),
            version_group: version_groups
                .get(&self.version_group_id)
                .unwrap_or_else(|| {
                    panic!(
                        "No version group '{}' found for version '{}'",
                        self.version_group_id, self.id
                    )
                })
                .clone(),
        };
        Arc::new(version)
    }
}

#[derive(Debug)]
pub struct Version {
    pub id: VersionId,
    pub identifier: String,
    pub names: LocalizedNames,
    pub version_group: Arc<VersionGroup>,
}
