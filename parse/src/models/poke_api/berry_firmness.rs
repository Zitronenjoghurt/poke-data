use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::berry_firmness::{BerryFirmness, BerryFirmnessId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BerryFirmnessData {
    id: BerryFirmnessId,
    identifier: String,
}

impl PokeApiModel for BerryFirmnessData {
    fn file_name() -> &'static str {
        "berry_firmness"
    }
}

impl HasId for BerryFirmnessData {
    type Id = BerryFirmnessId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<BerryFirmness> for BerryFirmnessData {
    fn into_model(self, data: &RawData) -> BerryFirmness {
        BerryFirmness {
            id: self.id,
            identifier: self.identifier,
            names: data.berry_firmness_names.get_model(&self.id, data),
        }
    }
}
