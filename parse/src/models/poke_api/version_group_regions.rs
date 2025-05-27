use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::region::RegionId;
use poke_data::models::version_group::VersionGroupId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionGroupRegionData {
    pub version_group_id: VersionGroupId,
    pub region_id: RegionId,
}

impl PokeApiModel for VersionGroupRegionData {
    fn file_name() -> &'static str {
        "version_group_regions"
    }
}

impl HasId for VersionGroupRegionData {
    type Id = VersionGroupId;

    fn id(&self) -> Self::Id {
        self.version_group_id
    }
}

impl IntoModel<Vec<RegionId>> for Vec<VersionGroupRegionData> {
    fn into_model(self, _data: &RawData) -> Vec<RegionId> {
        self.into_iter().map(|v| v.region_id).collect()
    }
}
