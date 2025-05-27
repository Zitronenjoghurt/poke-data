use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::generation::GenerationId;
use poke_data::models::version_group::{UnlinkedVersionGroup, VersionGroupId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionGroupData {
    id: VersionGroupId,
    identifier: String,
    generation_id: GenerationId,
    order: u8,
}

impl PokeApiModel for VersionGroupData {
    fn file_name() -> &'static str {
        "version_groups"
    }
}

impl HasId for VersionGroupData {
    type Id = VersionGroupId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<UnlinkedVersionGroup> for VersionGroupData {
    fn into_model(self, data: &RawData) -> UnlinkedVersionGroup {
        UnlinkedVersionGroup {
            id: self.id,
            identifier: self.identifier,
            generation_id: self.generation_id,
            region_ids: data.version_group_regions.get_model(&self.id, data),
            move_method_ids: data.version_group_move_methods.get_model(&self.id, data),
            order: self.order,
        }
    }
}
