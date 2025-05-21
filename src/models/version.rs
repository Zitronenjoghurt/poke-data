use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::localized_names::LocalizedStrings;
use crate::models::version_group::{VersionGroup, VersionGroupId};
use crate::traits::has_identifier::HasIdentifier;
use crate::traits::has_localized_names::HasLocalizedNames;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type VersionId = u16;

#[derive(Debug)]
pub struct Version {
    pub id: VersionId,
    pub identifier: String,
    pub names: LocalizedStrings,
    pub version_group: Arc<VersionGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedVersion {
    pub id: VersionId,
    pub identifier: String,
    pub names: LocalizedStrings,
    pub version_group_id: VersionGroupId,
}

impl Linkable for UnlinkedVersion {
    type Linked = Arc<Version>;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let version_group = context
            .version_groups
            .get(&self.version_group_id)
            .unwrap_or_else(|| {
                panic!(
                    "No version group '{}' found for version '{}'",
                    self.version_group_id, self.id
                )
            })
            .clone();

        let version = Version {
            id: self.id,
            identifier: self.identifier.clone(),
            names: self.names.clone(),
            version_group,
        };

        Arc::new(version)
    }
}

impl HasLocalizedNames for Version {
    fn localized_names(&self) -> &LocalizedStrings {
        &self.names
    }
}

impl HasIdentifier for Version {
    fn identifier(&self) -> &str {
        &self.identifier
    }
}
