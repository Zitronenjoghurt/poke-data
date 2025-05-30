use crate::collections::abilities::AbilitiesCollection;
use crate::collections::berries::BerriesCollection;
use crate::collections::egg_groups::EggGroupsCollection;
use crate::collections::generations::GenerationsCollection;
use crate::collections::item_categories::ItemCategoriesCollection;
use crate::collections::items::ItemsCollection;
use crate::collections::location_areas::LocationAreasCollection;
use crate::collections::locations::LocationsCollection;
use crate::collections::moves::MovesCollection;
use crate::collections::pokedexes::PokedexCollection;
use crate::collections::pokemon::PokemonCollection;
use crate::collections::pokemon_type_efficacies::PokemonTypeEfficaciesCollection;
use crate::collections::regions::RegionsCollection;
use crate::collections::species::SpeciesCollection;
use crate::collections::versions::VersionsCollection;
use crate::INCLUDED_DATA;
use std::path::Path;
use unlinked::UnlinkedPokeData;

pub mod link_context;
pub mod linkable;
pub mod unlinked;

pub struct PokeData {
    pub abilities: AbilitiesCollection,
    pub berries: BerriesCollection,
    pub egg_groups: EggGroupsCollection,
    pub generations: GenerationsCollection,
    pub items: ItemsCollection,
    pub item_categories: ItemCategoriesCollection,
    pub locations: LocationsCollection,
    pub location_areas: LocationAreasCollection,
    pub moves: MovesCollection,
    pub pokedexes: PokedexCollection,
    pub pokemon: PokemonCollection,
    pub regions: RegionsCollection,
    pub species: SpeciesCollection,
    pub versions: VersionsCollection,
    pub pokemon_type_efficacies: PokemonTypeEfficaciesCollection,
}

impl PokeData {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let decompressed_bytes = zstd::decode_all(INCLUDED_DATA)?;
        PokeData::load_from_bytes(&decompressed_bytes)
    }

    pub fn load_from_path(compressed_data_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let unlinked_data = UnlinkedPokeData::load(compressed_data_path)?;
        Ok(unlinked_data.initialize())
    }

    pub fn load_from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let unlinked_data = UnlinkedPokeData::from_bytes(bytes)?;
        Ok(unlinked_data.initialize())
    }
}
