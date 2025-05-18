use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::encounter_method::{EncounterMethod, EncounterMethodId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterMethodData {
    id: EncounterMethodId,
    identifier: String,
    order: u8,
}

impl PokeApiModel for EncounterMethodData {
    fn file_name() -> &'static str {
        "encounter_methods"
    }
}

impl HasId for EncounterMethodData {
    type Id = EncounterMethodId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<EncounterMethod> for EncounterMethodData {
    fn into_model(self, data: &RawData) -> EncounterMethod {
        EncounterMethod {
            id: self.id,
            identifier: self.identifier,
            names: data.encounter_method_prose.get_model(&self.id, data),
            order: self.order,
        }
    }
}
