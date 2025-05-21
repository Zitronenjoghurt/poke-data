use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedString;
use poke_data::models::item_category::ItemCategoryId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCategoryProseData {
    item_category_id: ItemCategoryId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for ItemCategoryProseData {
    fn file_name() -> &'static str {
        "item_category_prose"
    }
}

impl HasId for ItemCategoryProseData {
    type Id = ItemCategoryId;

    fn id(&self) -> Self::Id {
        self.item_category_id
    }
}

impl HasLocalizedString for ItemCategoryProseData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn string(&self) -> String {
        self.name.clone()
    }
}
