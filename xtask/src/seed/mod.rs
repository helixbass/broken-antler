use std::collections::HashMap;
use std::path::PathBuf;

use chrono::NaiveDate;
use geoutils::Location;
use juriji::{insert_events, EventForInsertion};
use serde::Deserialize;
use shared::{get_db_pool, get_mutex_guard, Event, Show, Song, Venue};
use sqlx::{Pool, Postgres};
use tokio::fs::read_to_string;
use uuid::Uuid;

use crate::{parse_json_file, workspace_root_directory};

pub async fn seed() -> anyhow::Result<()> {
    let db_pool = get_db_pool().await.unwrap();
    create_tables(&db_pool).await?;
    let venue_slugs = seed_venues(&db_pool).await?;
    seed_songs(&db_pool).await?;
    seed_shows(&db_pool, &venue_slugs).await?;
    unimplemented!()
}

async fn seed_venues(db_pool: &Pool<Postgres>) -> anyhow::Result<HashMap<String, Uuid>> {
    let venues_json: Vec<VenueJson> = parse_json_file("venues").await?;

    let venue_slugs: HashMap<_, _> = venues_json
        .iter()
        .map(|venue| (venue.slug.clone(), venue.id))
        .collect();

    let venues: Vec<Venue> = venues_json.into_iter().map(Into::into).collect();
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

    Ok(venue_slugs)
}

#[derive(Deserialize)]
struct VenueJson {
    pub id: Uuid,
    pub name: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub slug: String,
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

async fn seed_shows(
    db_pool: &Pool<Postgres>,
    venue_slugs: &HashMap<String, Uuid>,
) -> anyhow::Result<()> {
    let shows: Vec<Show> = parse_json_file::<Vec<ShowJson>>("shows")
        .await?
        .into_iter()
        .map(|show| show.process(venue_slugs))
        .collect();
    println!("shows: {shows:#?}");

    insert_events(
        shows
            .into_iter()
            .map(|show| Event::InsertShow(show))
            .map(|event| EventForInsertion::from(&event)),
        get_mutex_guard().await,
        db_pool,
    )
    .await;

    Ok(())
}

#[derive(Deserialize)]
struct ShowJson {
    pub id: Uuid,
    pub date: NaiveDate,
    pub venue: VenueOnlySlug,
}

#[derive(Deserialize)]
struct VenueOnlySlug {
    pub slug: String,
}

impl ShowJson {
    pub fn process(self, venue_slugs: &HashMap<String, Uuid>) -> Show {
        Show {
            id: self.id,
            date: self.date,
            venue_id: venue_slugs[&self.venue.slug],
        }
    }
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
