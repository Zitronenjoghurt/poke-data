use std::error::Error;
use std::path::{Path, PathBuf};

pub mod poke_api;

pub trait RemoteModel: Sized {
    fn source_url() -> String;
    fn source_file_path(base_path: &Path) -> PathBuf;
    fn parse(content: &str) -> Result<Vec<Self>, Box<dyn Error>>;

    async fn fetch_content(base_path: &Path) -> Result<String, Box<dyn Error>> {
        let source_file_path = Self::source_file_path(base_path);
        if !source_file_path.exists() {
            std::fs::create_dir_all(source_file_path.parent().unwrap())?;
            let client = reqwest::Client::new();
            let response = client.get(Self::source_url()).send().await?;
            let status = response.status();
            if !status.is_success() {
                return Err(format!(
                    "Request for source file path {} failed with status {}",
                    source_file_path.display(),
                    status
                )
                .into());
            }
            let content = response.text().await?;
            std::fs::write(source_file_path, content.clone())?;
            Ok(content)
        } else {
            let content = std::fs::read_to_string(source_file_path)?;
            Ok(content)
        }
    }

    async fn load(base_path: &Path) -> Result<Vec<Self>, Box<dyn Error>> {
        let content = Self::fetch_content(base_path).await?;
        Self::parse(&content)
    }
}
