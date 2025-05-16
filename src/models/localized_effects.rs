use crate::models::language::LanguageId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalizedEffects(HashMap<LanguageId, LocalizedEffect>);

impl LocalizedEffects {
    pub fn new(effects: HashMap<LanguageId, LocalizedEffect>) -> Self {
        Self(effects)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalizedEffect {
    pub effect: String,
    pub short_effect: String,
}
