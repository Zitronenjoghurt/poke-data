use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::region::{Region, RegionId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionData {
    id: RegionId,
    identifier: String,
}

impl PokeApiModel for RegionData {
    fn file_name() -> &'static str {
        "regions"
    }
}

impl HasId for RegionData {
    type Id = RegionId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<Region> for RegionData {
    fn into_model(self, data: &RawData) -> Region {
        Region {
            id: self.id,
            identifier: self.identifier,
            names: data.region_names.get_model(&self.id, data),
        }
    }
}
