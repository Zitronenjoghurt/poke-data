use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::version::{UnlinkedVersion, VersionId};
use poke_data::models::version_group::VersionGroupId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionData {
    id: VersionId,
    identifier: String,
    version_group_id: VersionGroupId,
}

impl PokeApiModel for VersionData {
    fn file_name() -> &'static str {
        "versions"
    }
}

impl HasId for VersionData {
    type Id = VersionId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<UnlinkedVersion> for VersionData {
    fn into_model(self, data: &RawData) -> UnlinkedVersion {
        UnlinkedVersion {
            id: self.id,
            identifier: self.identifier,
            names: data.version_names.get_model(&self.id, data),
            version_group_id: self.version_group_id,
        }
    }
}
