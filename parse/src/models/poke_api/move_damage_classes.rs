use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::damage_class::{DamageClass, DamageClassId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageClassData {
    id: DamageClassId,
    identifier: String,
}

impl PokeApiModel for DamageClassData {
    fn file_name() -> &'static str {
        "move_damage_classes"
    }
}

impl HasId for DamageClassData {
    type Id = DamageClassId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<DamageClass> for DamageClassData {
    fn into_model(self, data: &RawData) -> DamageClass {
        DamageClass {
            id: self.id,
            identifier: self.identifier,
            prose: data.damage_class_prose.get_model(&self.id, data),
        }
    }
}
