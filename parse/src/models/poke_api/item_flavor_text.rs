use crate::models::poke_api::PokeApiModel;
use crate::traits::has_flavor_text::HasFlavorText;
use crate::traits::has_id::HasId;
use poke_data::models::item::ItemId;
use poke_data::models::language::LanguageId;
use poke_data::models::version_group::VersionGroupId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemFlavorTextData {
    item_id: ItemId,
    version_group_id: VersionGroupId,
    language_id: LanguageId,
    flavor_text: String,
}

impl PokeApiModel for ItemFlavorTextData {
    fn file_name() -> &'static str {
        "item_flavor_text"
    }
}

impl HasId for ItemFlavorTextData {
    type Id = ItemId;

    fn id(&self) -> Self::Id {
        self.item_id
    }
}

impl HasFlavorText for ItemFlavorTextData {
    fn language(&self) -> LanguageId {
        self.language_id
    }

    fn version_group(&self) -> VersionGroupId {
        self.version_group_id
    }

    fn text(&self) -> String {
        self.flavor_text.clone()
    }
}
