use crate::models::language::LanguageId;
use crate::models::localized_names::LocalizedStrings;
use crate::types::language::Language;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedNameDescriptions(HashMap<LanguageId, LocalizedNameDescription>);

impl LocalizedNameDescriptions {
    pub fn new(localizations: HashMap<LanguageId, LocalizedNameDescription>) -> Self {
        Self(localizations)
    }

    pub fn get_by_language(&self, language: Language) -> Option<&LocalizedNameDescription> {
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

    pub fn localizations(&self) -> &HashMap<LanguageId, LocalizedNameDescription> {
        &self.0
    }

    pub fn get_names(&self) -> LocalizedStrings {
        let localizations = self
            .0
            .iter()
            .map(|(k, v)| (k.clone(), v.name.clone()))
            .collect();
        LocalizedStrings::new(localizations)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedNameDescription {
    pub name: String,
    pub description: String,
}
