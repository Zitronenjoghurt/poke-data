use crate::models::language::LanguageId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedNames(HashMap<LanguageId, String>);

impl LocalizedNames {
    pub fn new(localizations: HashMap<LanguageId, String>) -> Self {
        Self(localizations)
    }
}
