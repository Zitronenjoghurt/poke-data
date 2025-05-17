use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::growth_rate::{GrowthRateExperience, GrowthRateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceData {
    pub growth_rate_id: GrowthRateId,
    pub level: u8,
    pub experience: u32,
}

impl PokeApiModel for ExperienceData {
    fn file_name() -> &'static str {
        "experience"
    }
}

impl HasId for ExperienceData {
    type Id = GrowthRateId;

    fn id(&self) -> GrowthRateId {
        self.growth_rate_id
    }
}

impl IntoModel<GrowthRateExperience> for Vec<ExperienceData> {
    fn into_model(self, _data: &RawData) -> GrowthRateExperience {
        let experiences = self
            .iter()
            .map(|data| (data.level, data.experience))
            .collect();
        GrowthRateExperience::new(experiences)
    }
}
