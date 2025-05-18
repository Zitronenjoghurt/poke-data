use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name_description::HasLocalizedNameDescription;
use poke_data::models::damage_class::DamageClassId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageClassProseData {
    move_damage_class_id: DamageClassId,
    local_language_id: LanguageId,
    name: String,
    description: String,
}

impl PokeApiModel for DamageClassProseData {
    fn file_name() -> &'static str {
        "move_damage_class_prose"
    }
}

impl HasId for DamageClassProseData {
    type Id = DamageClassId;

    fn id(&self) -> Self::Id {
        self.move_damage_class_id
    }
}

impl HasLocalizedNameDescription for DamageClassProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}
