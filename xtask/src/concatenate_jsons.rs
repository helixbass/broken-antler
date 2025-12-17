use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;
use squalid::_d;
use tokio::fs;

use crate::{json_seed_file_directory, json_seed_file_path, parse_json_file};

pub async fn concatenate_jsons(file_name_root: &str) -> anyhow::Result<()> {
    let mut all_json_file_paths = fs::read_dir(json_seed_file_directory()).await.unwrap();
    let regex = Regex::new(&format!(r#"^{file_name_root}-\d+\.json"#)).unwrap();
    let mut parsed_files: HashMap<String, serde_json::Value> = _d();
    while let Some(dir_entry) = all_json_file_paths.next_entry().await.unwrap() {
        let file_name_str = dir_entry.file_name().into_string().unwrap();
        if regex.is_match(&file_name_str) {
            let one_file_json: serde_json::Value =
                parse_json_file(&file_name_str.split(".").next().unwrap())
                    .await
                    .unwrap();
            parsed_files.insert(file_name_str, one_file_json);
        }
    }
    assert!(!parsed_files.is_empty());
    fs::write(
        json_seed_file_path(file_name_root),
        serde_json::to_string_pretty(&serde_json::Value::Array(
            parsed_files
                .into_iter()
                .sorted_by_key(|(file_name, _)| file_name.clone())
                .flat_map(|(_, one_file_json)| {
                    one_file_json
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .collect(),
        ))
        .unwrap(),
    )
    .await
    .unwrap();

    Ok(())
}
