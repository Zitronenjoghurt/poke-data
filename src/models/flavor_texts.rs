use crate::models::language::LanguageId;
use crate::models::version_group::VersionGroupId;
use crate::types::language::Language;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlavorTexts(HashMap<LanguageId, Vec<FlavorText>>);

impl FlavorTexts {
    pub fn new(texts: HashMap<LanguageId, Vec<FlavorText>>) -> Self {
        Self(texts)
    }

    pub fn get_by_language(&self, language: Language) -> Option<&Vec<FlavorText>> {
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
pub struct FlavorText {
    pub version_group_id: VersionGroupId,
    pub flavor_text: String,
}
