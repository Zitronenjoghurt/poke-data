use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::contest_type::{ContestType, ContestTypeId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContestTypeData {
    id: ContestTypeId,
    identifier: String,
}

impl PokeApiModel for ContestTypeData {
    fn file_name() -> &'static str {
        "contest_types"
    }
}

impl HasId for ContestTypeData {
    type Id = ContestTypeId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<ContestType> for ContestTypeData {
    fn into_model(self, data: &RawData) -> ContestType {
        ContestType {
            id: self.id,
            identifier: self.identifier,
            names: data.contest_type_names.get_model(&self.id, data),
        }
    }
}
