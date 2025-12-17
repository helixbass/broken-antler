use std::path::PathBuf;

use geoutils::Location;
use juriji::{insert_events, EventForInsertion};
use serde::Deserialize;
use shared::{get_db_pool, get_mutex_guard, Event, Song, Venue};
use sqlx::{Pool, Postgres};
use tokio::fs::read_to_string;
use uuid::Uuid;

use crate::{parse_json_file, workspace_root_directory};

pub async fn seed() -> anyhow::Result<()> {
    let db_pool = get_db_pool().await.unwrap();
    create_tables(&db_pool).await?;
    seed_venues(&db_pool).await?;
    seed_songs(&db_pool).await?;
    unimplemented!()
}

async fn seed_venues(db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let venues: Vec<Venue> = parse_json_file::<Vec<VenueJson>>("venues")
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    println!("venues: {venues:#?}");

    insert_events(
        venues
            .into_iter()
            .map(|venue| Event::InsertVenue(venue))
            .map(|event| EventForInsertion::from(&event)),
        get_mutex_guard().await,
        db_pool,
    )
    .await;

    Ok(())
}

#[derive(Deserialize)]
struct VenueJson {
    pub id: Uuid,
    pub name: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl From<VenueJson> for Venue {
    fn from(value: VenueJson) -> Self {
        Self {
            id: value.id,
            name: value.name,
            location: match (value.latitude, value.longitude) {
                (Some(latitude), Some(longitude)) => Some(Location::new(latitude, longitude)),
                (None, None) => None,
                _ => unreachable!(),
            },
        }
    }
}

async fn seed_songs(db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let songs: Vec<Song> = parse_json_file::<Vec<Song>>("songs")
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    println!("songs: {songs:#?}");

    insert_events(
        songs
            .into_iter()
            .map(|song| Event::InsertSong(song))
            .map(|event| EventForInsertion::from(&event)),
        get_mutex_guard().await,
        db_pool,
    )
    .await;

    Ok(())
}

fn sql_file_path(file_name_root: &str) -> PathBuf {
    let mut path = workspace_root_directory();
    path.push(&format!("sql/{file_name_root}.sql"));
    path
}

async fn create_tables(db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let sql = read_to_string(sql_file_path("create_tables")).await?;
    for command in sql.split("\n\n") {
        sqlx::query(command).execute(db_pool).await?;
    }
    Ok(())
}
