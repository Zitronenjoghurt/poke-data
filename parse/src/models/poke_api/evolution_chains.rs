use crate::models::poke_api::PokeApiModel;
use crate::traits::has_id::HasId;
use poke_data::models::item::ItemId;
use serde::{Deserialize, Serialize};

pub type EvolutionChainId = u16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionChainData {
    id: EvolutionChainId,
    pub baby_trigger_item_id: Option<ItemId>,
}

impl PokeApiModel for EvolutionChainData {
    fn file_name() -> &'static str {
        "evolution_chains"
    }
}

impl HasId for EvolutionChainData {
    type Id = EvolutionChainId;

    fn id(&self) -> Self::Id {
        self.id
    }
}
