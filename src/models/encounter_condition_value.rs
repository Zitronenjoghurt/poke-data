use crate::data::link_context::LinkContext;
use crate::data::linkable::Linkable;
use crate::models::encounter_condition::{EncounterCondition, EncounterConditionId};
use crate::models::localized_names::LocalizedStrings;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type EncounterConditionValueId = u16;

#[derive(Debug)]
pub struct EncounterConditionValue {
    pub id: EncounterConditionValueId,
    pub identifier: String,
    pub names: LocalizedStrings,
    pub is_default: bool,
    pub encounter_condition: Arc<EncounterCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkedEncounterConditionValue {
    pub id: EncounterConditionValueId,
    pub identifier: String,
    pub names: LocalizedStrings,
    pub is_default: bool,
    pub encounter_condition_id: EncounterConditionId,
}

impl Linkable for UnlinkedEncounterConditionValue {
    type Linked = Arc<EncounterConditionValue>;

    fn link(&self, context: &LinkContext) -> Self::Linked {
        let encounter_condition = context
            .encounter_conditions
            .get(&self.encounter_condition_id)
            .unwrap_or_else(|| {
                panic!(
                    "No encounter condition '{}' found for encounter condition value '{}'",
                    self.encounter_condition_id, self.id
                )
            })
            .clone();

        let encounter_condition_value = EncounterConditionValue {
            id: self.id,
            identifier: self.identifier.clone(),
            names: self.names.clone(),
            is_default: self.is_default,
            encounter_condition,
        };

        Arc::new(encounter_condition_value)
    }
}
