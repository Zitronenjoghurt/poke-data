use crate::models::language::LanguageId;
use crate::models::version_group::VersionGroupId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct FlavorTexts(HashMap<LanguageId, Vec<FlavorText>>);

impl FlavorTexts {
    pub fn new(texts: HashMap<LanguageId, Vec<FlavorText>>) -> Self {
        Self(texts)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlavorText {
    pub version_group_id: VersionGroupId,
    pub flavor_text: String,
}
