use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use crate::traits::into_model::IntoModel;
use crate::RawData;
use poke_data::models::item::ItemId;
use poke_data::models::pokemon::wild_item::{UnlinkedPokemonWildItem, UnlinkedPokemonWildItems};
use poke_data::models::pokemon::PokemonId;
use poke_data::models::version::VersionId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonItemData {
    pokemon_id: PokemonId,
    version_id: VersionId,
    item_id: ItemId,
    rarity: u8,
}

impl PokeApiModel for PokemonItemData {
    fn file_name() -> &'static str {
        "pokemon_items"
    }
}

impl HasId for PokemonItemData {
    type Id = PokemonId;

    fn id(&self) -> Self::Id {
        self.pokemon_id
    }
}

impl IntoModel<UnlinkedPokemonWildItem> for PokemonItemData {
    fn into_model(self, _data: &RawData) -> UnlinkedPokemonWildItem {
        UnlinkedPokemonWildItem {
            item_id: self.item_id,
            rarity: self.rarity,
        }
    }
}

impl IntoModel<UnlinkedPokemonWildItems> for Vec<PokemonItemData> {
    fn into_model(self, data: &RawData) -> UnlinkedPokemonWildItems {
        let items = self
            .iter()
            .map(|entry| (entry.version_id, entry.clone().into_model(data)))
            .fold(
                HashMap::new(),
                |mut acc: HashMap<VersionId, Vec<UnlinkedPokemonWildItem>>, (version_id, item)| {
                    acc.entry(version_id).or_default().push(item);
                    acc
                },
            );
        UnlinkedPokemonWildItems::new(items)
    }
}
