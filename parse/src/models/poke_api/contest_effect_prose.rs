use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::raw_data::RawData;
use poke_data::models::contest_effect::{
    ContestEffectId, ContestEffectProse, ContestEffectProseEntry,
};
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContestEffectProseData {
    contest_effect_id: ContestEffectId,
    local_language_id: LanguageId,
    flavor_text: String,
    effect: String,
}

impl PokeApiModel for ContestEffectProseData {
    fn file_name() -> &'static str {
        "contest_effect_prose"
    }
}

impl HasId for ContestEffectProseData {
    type Id = ContestEffectId;

    fn id(&self) -> Self::Id {
        self.contest_effect_id
    }
}

impl IntoModel<ContestEffectProseEntry> for ContestEffectProseData {
    fn into_model(self, _data: &RawData) -> ContestEffectProseEntry {
        ContestEffectProseEntry {
            flavor_text: self.flavor_text.to_string(),
            effect: self.effect.to_string(),
        }
    }
}

impl IntoModel<ContestEffectProse> for Vec<ContestEffectProseData> {
    fn into_model(self, data: &RawData) -> ContestEffectProse {
        let entries = self
            .iter()
            .map(|entry| (entry.local_language_id, entry.clone().into_model(data)))
            .collect();
        ContestEffectProse::new(entries)
    }
}
