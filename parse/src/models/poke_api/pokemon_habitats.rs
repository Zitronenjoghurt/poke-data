use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::habitat::{Habitat, HabitatId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitatData {
    id: HabitatId,
    identifier: String,
}

impl PokeApiModel for HabitatData {
    fn file_name() -> &'static str {
        "pokemon_habitats"
    }
}

impl HasId for HabitatData {
    type Id = HabitatId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<Habitat> for HabitatData {
    fn into_model(self, data: &RawData) -> Habitat {
        Habitat {
            id: self.id,
            identifier: self.identifier,
            names: data.habitat_names.get_model(&self.id, data),
        }
    }
}
