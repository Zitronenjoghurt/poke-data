use poke_data::assets::PokemonAssets;
use poke_data::models::png_asset::PngAsset;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::path::Path;
use std::str::FromStr;

pub fn load_assets(base_path: &Path) -> Result<PokemonAssets, Box<dyn Error>> {
    let sprites_path = base_path.join("sprites").join("sprites");
    let pokemon_sprites_path = sprites_path.join("pokemon");
    let official_artworks_path = pokemon_sprites_path.join("other").join("official-artwork");

    Ok(PokemonAssets {
        official_artworks: load_compressed_pngs(&official_artworks_path)?,
    })
}

fn load_compressed_pngs<K>(directory: &Path) -> Result<HashMap<K, PngAsset>, Box<dyn Error>>
where
    K: FromStr + Eq + Hash,
{
    let mut map = HashMap::new();

    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() || path.extension() != Some("png".as_ref()) {
            continue;
        }

        let filename = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("Invalid filename")?;

        let Some(key) = K::from_str(filename).ok() else {
            continue;
        };

        let png_data = std::fs::read(&path)?;
        let compressed = PngAsset::new(&png_data)?;
        map.insert(key, compressed);
    }

    Ok(map)
}
