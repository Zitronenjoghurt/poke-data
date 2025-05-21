use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedString;
use poke_data::models::item::ItemId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemNameData {
    item_id: ItemId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for ItemNameData {
    fn file_name() -> &'static str {
        "item_names"
    }
}

impl HasId for ItemNameData {
    type Id = ItemId;

    fn id(&self) -> Self::Id {
        self.item_id
    }
}

impl HasLocalizedString for ItemNameData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn string(&self) -> String {
        self.name.clone()
    }
}
