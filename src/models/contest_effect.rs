use crate::models::language::LanguageId;
use crate::types::language::Language;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ContestEffectId = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContestEffect {
    pub id: ContestEffectId,
    pub appeal: u8,
    pub jam: u8,
    pub prose: ContestEffectProse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContestEffectProse(HashMap<LanguageId, ContestEffectProseEntry>);

impl ContestEffectProse {
    pub fn new(localizations: HashMap<LanguageId, ContestEffectProseEntry>) -> Self {
        Self(localizations)
    }

    pub fn get_by_language(&self, language: Language) -> Option<&ContestEffectProseEntry> {
        let language_id = language as LanguageId;
        if let Some(target) = self.0.get(&language_id) {
            return Some(target);
        }

        let default_language_id = Language::default() as LanguageId;
        if let Some(default) = self.0.get(&default_language_id) {
            return Some(default);
        }

        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContestEffectProseEntry {
    pub flavor_text: String,
    pub effect: String,
}
