use regex::Regex;
use tokio::fs;

use crate::{json_seed_file_directory, json_seed_file_path, parse_json_file};

pub async fn concatenate_jsons(file_name_root: &str) -> anyhow::Result<()> {
    let mut all_json_file_paths = fs::read_dir(json_seed_file_directory()).await.unwrap();
    let regex = Regex::new(&format!(r#"^{file_name_root}-\d+\.json"#)).unwrap();
    let mut concatenated = serde_json::Value::Array(vec![]);
    while let Some(dir_entry) = all_json_file_paths.next_entry().await.unwrap() {
        if regex.is_match(&dir_entry.file_name().into_string().unwrap()) {
            let one_file_json: serde_json::Value = parse_json_file(file_name_root).await.unwrap();
            concatenated
                .as_array_mut()
                .unwrap()
                .extend(one_file_json.as_array().unwrap().into_iter().cloned());
        }
    }
    assert!(!concatenated.as_array().unwrap().is_empty());
    fs::write(
        json_seed_file_path(file_name_root),
        serde_json::to_string_pretty(&concatenated).unwrap(),
    )
    .await
    .unwrap();

    Ok(())
}
