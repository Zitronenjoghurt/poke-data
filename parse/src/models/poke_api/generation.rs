use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::generation::{GenerationId, UnlinkedGeneration};
use poke_data::models::region::RegionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationData {
    id: GenerationId,
    main_region_id: RegionId,
    identifier: String,
}

impl PokeApiModel for GenerationData {
    fn file_name() -> &'static str {
        "generations"
    }
}

impl HasId for GenerationData {
    type Id = GenerationId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<UnlinkedGeneration> for GenerationData {
    fn into_model(self, data: &RawData) -> UnlinkedGeneration {
        UnlinkedGeneration {
            id: self.id,
            identifier: self.identifier,
            main_region_id: self.main_region_id,
            names: data.generation_names.get_model(&self.id, data),
        }
    }
}
