use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::has_localized_name::HasLocalizedString;
use poke_data::models::item_pocket::ItemPocketId;
use poke_data::models::language::LanguageId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPocketNameData {
    item_pocket_id: ItemPocketId,
    local_language_id: LanguageId,
    name: String,
}

impl PokeApiModel for ItemPocketNameData {
    fn file_name() -> &'static str {
        "item_pocket_names"
    }
}

impl HasId for ItemPocketNameData {
    type Id = ItemPocketId;

    fn id(&self) -> Self::Id {
        self.item_pocket_id
    }
}

impl HasLocalizedString for ItemPocketNameData {
    fn language(&self) -> LanguageId {
        self.local_language_id
    }

    fn string(&self) -> String {
        self.name.clone()
    }
}
