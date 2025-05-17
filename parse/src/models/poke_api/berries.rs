use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::berry::{BerryId, UnlinkedBerry};
use poke_data::models::berry_firmness::BerryFirmnessId;
use poke_data::models::item::ItemId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BerryData {
    id: BerryId,
    item_id: ItemId,
    firmness_id: BerryFirmnessId,
    size: u16,
    max_harvest: u8,
    growth_time: u8,
    soil_dryness: u8,
    smoothness: u8,
}

impl PokeApiModel for BerryData {
    fn file_name() -> &'static str {
        "berries"
    }
}

impl HasId for BerryData {
    type Id = BerryId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<UnlinkedBerry> for BerryData {
    fn into_model(self, _data: &RawData) -> UnlinkedBerry {
        UnlinkedBerry {
            id: self.id,
            item_id: self.item_id,
            firmness_id: self.firmness_id,
            size: self.size,
            max_harvest: self.max_harvest,
            growth_time: self.growth_time,
            soil_dryness: self.soil_dryness,
            smoothness: self.smoothness,
        }
    }
}
