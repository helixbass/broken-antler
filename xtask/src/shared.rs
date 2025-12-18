use std::path::PathBuf;

use serde::de::DeserializeOwned;
use tokio::fs::read_to_string;

pub fn workspace_root_directory() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("..");
    path
}

pub fn json_seed_file_directory() -> PathBuf {
    let mut path = workspace_root_directory();
    path.push("phishin-seed");
    path
}

pub fn json_seed_file_path(file_name_root: &str) -> PathBuf {
    let mut path = json_seed_file_directory();
    path.push(&format!("{file_name_root}.json"));
    path
}

pub async fn parse_json_file<TTarget: DeserializeOwned>(
    file_name_root: &str,
) -> anyhow::Result<TTarget> {
    Ok(serde_json::from_str(
        &read_to_string(json_seed_file_path(file_name_root)).await?,
    )?)
}
