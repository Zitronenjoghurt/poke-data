use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::egg_group::{EggGroup, EggGroupId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EggGroupData {
    id: EggGroupId,
    identifier: String,
}

impl PokeApiModel for EggGroupData {
    fn file_name() -> &'static str {
        "egg_groups"
    }
}

impl HasId for EggGroupData {
    type Id = EggGroupId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<EggGroup> for EggGroupData {
    fn into_model(self, data: &RawData) -> EggGroup {
        EggGroup {
            id: self.id,
            identifier: self.identifier,
            names: data.egg_group_prose.get_model(&self.id, data),
        }
    }
}
