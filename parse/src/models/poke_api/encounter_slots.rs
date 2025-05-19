use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use poke_data::models::egg_group::EggGroupId;
use poke_data::models::encounter_method::EncounterMethodId;
use poke_data::models::version_group::VersionGroupId;
use serde::{Deserialize, Serialize};

pub type EncounterSlotId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterSlotData {
    pub id: EncounterSlotId,
    pub version_group_id: VersionGroupId,
    pub encounter_method_id: EncounterMethodId,
    pub slot: Option<u8>,
    pub rarity: u8,
}

impl PokeApiModel for EncounterSlotData {
    fn file_name() -> &'static str {
        "encounter_slots"
    }
}

impl HasId for EncounterSlotData {
    type Id = EggGroupId;

    fn id(&self) -> Self::Id {
        self.id
    }
}
