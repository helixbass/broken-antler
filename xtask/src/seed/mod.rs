use std::path::PathBuf;

use serde::de::DeserializeOwned;
use shared::{get_db_pool, Venue};
use sqlx::{Pool, Postgres};
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
    let db_pool = get_db_pool().await.unwrap();
    seed_venues(&db_pool).await?;
    unimplemented!()
}

async fn seed_venues(db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let venues: Vec<Venue> = parse_json_file("venues").await?;
    println!("venues: {venues:#?}");

    unimplemented!()
}
