use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::berry::BerryId;
use poke_data::models::berry_flavor::BerryFlavorId;
use poke_data::models::contest_type::ContestTypeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BerryFlavorData {
    berry_id: BerryId,
    contest_type_id: ContestTypeId,
    flavor: u8,
}

impl PokeApiModel for BerryFlavorData {
    fn file_name() -> &'static str {
        "berry_flavors"
    }
}

impl HasId for BerryFlavorData {
    type Id = BerryId;

    fn id(&self) -> Self::Id {
        self.berry_id
    }
}

impl IntoModel<Vec<(BerryFlavorId, u8)>> for Vec<BerryFlavorData> {
    fn into_model(self, _data: &RawData) -> Vec<(BerryFlavorId, u8)> {
        self.iter()
            .filter_map(|entry| {
                if entry.flavor > 0 {
                    Some((entry.contest_type_id, entry.flavor))
                } else {
                    None
                }
            })
            .collect()
    }
}
