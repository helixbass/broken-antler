use shared::Set;
use tokio::fs;
use uuid::Uuid;

use crate::{
    get_show_original_ids, get_song_performances_by_set, json_seed_file_path, parse_json_file,
    SongPerformanceJson,
};

pub async fn create_sets_json() -> anyhow::Result<()> {
    let song_performances: Vec<SongPerformanceJson> = parse_json_file("tracks").await?;

    let show_original_ids = get_show_original_ids().await?;

    let sets = get_song_performances_by_set(&song_performances)
        .into_iter()
        .flat_map(|(show_original_id, sets)| {
            sets.into_keys()
                .map(|set| Set {
                    id: Uuid::new_v4(),
                    show_id: show_original_ids[&show_original_id].clone(),
                    set_name: set,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fs::write(
        json_seed_file_path("sets"),
        serde_json::to_string_pretty(&sets).unwrap(),
    )
    .await
    .unwrap();

    Ok(())
}
