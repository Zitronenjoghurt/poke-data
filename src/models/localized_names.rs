use crate::models::language::LanguageId;
use crate::types::language::Language;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedNames(HashMap<LanguageId, String>);

impl LocalizedNames {
    pub fn new(localizations: HashMap<LanguageId, String>) -> Self {
        Self(localizations)
    }

    pub fn localizations(&self) -> &HashMap<LanguageId, String> {
        &self.0
    }

    pub fn get_by_language(&self, language: Language) -> &str {
        let language_id = language as LanguageId;
        if let Some(target) = self.0.get(&language_id) {
            return target;
        }

        let default_language_id = Language::default() as LanguageId;
        if let Some(default) = self.0.get(&default_language_id) {
            return default;
        }

        "NO TRANSLATIONS"
    }
}
