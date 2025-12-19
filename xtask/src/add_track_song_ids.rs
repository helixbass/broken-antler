use crate::{json_seed_file_path, parse_json_file};

use squalid::OptionExt;
use tokio::fs;
use uuid::Uuid;

pub async fn add_track_song_ids(file_name_root: &str) -> anyhow::Result<()> {
    let mut parsed = parse_json_file::<serde_json::Value>(file_name_root)
        .await
        .unwrap();
    parsed.as_array_mut().unwrap().into_iter().for_each(|item| {
        let item = item.as_object_mut().unwrap();
        for song in item
            .get_mut("songs")
            .unwrap()
            .as_array_mut()
            .unwrap()
            .iter_mut()
        {
            song.as_object_mut()
                .unwrap()
                .insert(
                    "id".to_owned(),
                    serde_json::Value::String(Uuid::new_v4().to_string()),
                )
                .assert_none();
        }
    });
    fs::write(
        json_seed_file_path(file_name_root),
        serde_json::to_string_pretty(&parsed).unwrap(),
    )
    .await
    .unwrap();

    Ok(())
}
