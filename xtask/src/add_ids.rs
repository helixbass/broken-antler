use crate::{json_seed_file_path, parse_json_file};

use tokio::fs;
use uuid::Uuid;

pub async fn add_ids(file_name_root: &str) -> anyhow::Result<()> {
    let initial_json: serde_json::Value = parse_json_file(file_name_root).await.unwrap();
    let mut nested_list = initial_json
        .as_object()
        .unwrap()
        .get(file_name_root)
        .unwrap()
        .clone();
    nested_list
        .as_array_mut()
        .unwrap()
        .into_iter()
        .for_each(|item| {
            item.as_object_mut().unwrap().insert(
                "id".to_owned(),
                serde_json::Value::String(Uuid::new_v4().to_string()),
            );
        });
    fs::write(
        json_seed_file_path(file_name_root),
        serde_json::to_string_pretty(&nested_list).unwrap(),
    )
    .await
    .unwrap();

    Ok(())
}
