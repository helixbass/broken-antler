use std::collections::HashMap;
use std::path::PathBuf;

use chrono::NaiveDate;
use itertools::Itertools;
use serde::{de::DeserializeOwned, Deserialize};
use tokio::fs::read_to_string;
use uuid::Uuid;

use shared::SetName;

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

#[derive(Debug, Deserialize)]
pub struct SongPerformanceJson {
    pub id: Uuid,
    pub set_name: SetName,
    pub show: ShowOnlyOriginalId,
    pub songs: Vec<SongOnlyId>,
}

#[derive(Debug, Deserialize)]
pub struct ShowOnlyOriginalId {
    #[serde(rename = "id")]
    pub original_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct SongOnlyId {
    pub id: Uuid,
}

pub fn get_song_performances_by_set(
    song_performances: &[SongPerformanceJson],
) -> HashMap<u32, HashMap<SetName, Vec<&SongPerformanceJson>>> {
    song_performances
        .iter()
        .into_group_map_by(|song_performance| song_performance.show.original_id)
        .into_iter()
        .map(|(show_original_id, song_performances)| {
            (
                show_original_id,
                song_performances
                    .into_iter()
                    .into_group_map_by(|song_performance| song_performance.set_name),
            )
        })
        .collect()
}

pub async fn get_show_original_ids() -> anyhow::Result<HashMap<u32, Uuid>> {
    let shows_json = parse_json_file::<Vec<ShowJson>>("shows").await?;

    Ok(shows_json
        .iter()
        .map(|show| (show.original_id, show.id))
        .collect())
}

#[derive(Deserialize)]
pub struct ShowJson {
    pub id: Uuid,
    pub original_id: u32,
    pub date: NaiveDate,
    pub venue: VenueOnlySlug,
}

#[derive(Deserialize)]
pub struct VenueOnlySlug {
    pub slug: String,
}
