use crate::models::poke_api::PokeApiModel;
use crate::traits::get_model::GetModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::growth_rate::{GrowthRate, GrowthRateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthRateData {
    id: GrowthRateId,
    identifier: String,
    formula: String,
}

impl PokeApiModel for GrowthRateData {
    fn file_name() -> &'static str {
        "growth_rates"
    }
}

impl HasId for GrowthRateData {
    type Id = GrowthRateId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl IntoModel<GrowthRate> for GrowthRateData {
    fn into_model(self, data: &RawData) -> GrowthRate {
        GrowthRate {
            id: self.id,
            identifier: self.identifier,
            formula: self.formula,
            names: data.growth_rate_prose.get_model(&self.id, data),
            experience: data.experiences.get_model(&self.id, data),
        }
    }
}
