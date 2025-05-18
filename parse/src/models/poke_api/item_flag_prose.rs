use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name_description::HasLocalizedNameDescription;
use poke_data::models::item_flag::ItemFlagId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemFlagProseData {
    item_flag_id: ItemFlagId,
    local_language_id: LanguageId,
    name: String,
    description: String,
}

impl PokeApiModel for ItemFlagProseData {
    fn file_name() -> &'static str {
        "item_flag_prose"
    }
}

impl HasId for ItemFlagProseData {
    type Id = ItemFlagId;

    fn id(&self) -> Self::Id {
        self.item_flag_id
    }
}

impl HasLocalizedNameDescription for ItemFlagProseData {
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
