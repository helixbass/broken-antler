use crate::{json_seed_file_path, parse_json_file};

use tokio::fs;

pub async fn add_ids(file_name_root: &str) -> anyhow::Result<()> {
    let initial_json: serde_json::Value = parse_json_file(file_name_root).await.unwrap();
    let nested_list = initial_json
        .as_object()
        .unwrap()
        .get(file_name_root)
        .unwrap();
    fs::write(
        json_seed_file_path(file_name_root),
        serde_json::to_string_pretty(nested_list).unwrap(),
    )
    .await
    .unwrap();

    Ok(())
}
