use std::path::PathBuf;

use serde::de::DeserializeOwned;
use tokio::fs::read_to_string;

fn workspace_root_directory() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("..");
    path
}

fn json_seed_file_path(file_name_root: &str) -> PathBuf {
    let mut path = workspace_root_directory();
    path.push(&format!("phishin-seed/{file_name_root}.json"));
    path
}

async fn parse_json_file<TTarget: DeserializeOwned>(
    file_name_root: &str,
) -> anyhow::Result<TTarget> {
    Ok(serde_json::from_str(
        &read_to_string(json_seed_file_path(file_name_root)).await?,
    )?)
}

pub async fn seed() -> anyhow::Result<()> {
    unimplemented!()
}
