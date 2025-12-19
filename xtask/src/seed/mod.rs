use std::collections::HashMap;
use std::path::PathBuf;

use geoutils::Location;
use juriji::{insert_events, EventForInsertion};
use serde::Deserialize;
use shared::{
    get_db_pool, get_mutex_guard, Event, Set, SetName, Show, Song, SongPerformance, Venue,
};
use sqlx::{Pool, Postgres};
use tokio::fs::read_to_string;
use uuid::Uuid;

use crate::{
    get_show_original_ids, get_song_performances_by_set, parse_json_file, workspace_root_directory,
    ShowJson, SongPerformanceJson,
};

pub async fn seed() -> anyhow::Result<()> {
    let db_pool = get_db_pool().await.unwrap();
    create_tables(&db_pool).await?;
    let venue_slugs = seed_venues(&db_pool).await?;
    seed_songs(&db_pool).await?;
    seed_shows(&db_pool, &venue_slugs).await?;
    let sets = seed_sets(&db_pool).await?;
    seed_song_performances(&sets, &db_pool).await?;
    unimplemented!()
}

async fn seed_venues(db_pool: &Pool<Postgres>) -> anyhow::Result<HashMap<String, Uuid>> {
    let venues_json: Vec<VenueJson> = parse_json_file("venues").await?;

    let venue_slugs: HashMap<_, _> = venues_json
        .iter()
        .map(|venue| (venue.slug.clone(), venue.id))
        .collect();

    let venues: Vec<Venue> = venues_json.into_iter().map(Into::into).collect();
    // println!("venues: {venues:#?}");

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
            name: value.name.into(),
            location: match (value.latitude, value.longitude) {
                (Some(latitude), Some(longitude)) => Some(Location::new(latitude, longitude)),
                (None, None) => None,
                _ => unreachable!(),
            },
        }
    }
}

async fn seed_songs(db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let songs: Vec<Song> = parse_json_file("songs").await?;
    // println!("songs: {songs:#?}");

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

    // println!("shows: {shows:#?}");

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

impl ShowJson {
    pub fn process(self, venue_slugs: &HashMap<String, Uuid>) -> Show {
        Show {
            id: self.id,
            date: self.date,
            venue_id: venue_slugs[&self.venue.slug],
        }
    }
}

async fn seed_sets(db_pool: &Pool<Postgres>) -> anyhow::Result<HashMap<(u32, SetName), Uuid>> {
    let sets: Vec<Set> = parse_json_file("sets").await?;

    let show_original_ids = get_show_original_ids()
        .await?
        .into_iter()
        .map(|(original_id, id)| (id, original_id))
        .collect::<HashMap<_, _>>();
    let sets_map: HashMap<(u32, SetName), Uuid> = sets
        .iter()
        .map(|set| ((show_original_ids[&set.show_id], set.set_name), set.id))
        .collect();

    insert_events(
        sets.into_iter()
            .map(|set| Event::InsertSet(set))
            .map(|event| EventForInsertion::from(&event)),
        get_mutex_guard().await,
        db_pool,
    )
    .await;

    Ok(sets_map)
}

async fn seed_song_performances(
    sets: &HashMap<(u32, SetName), Uuid>,
    db_pool: &Pool<Postgres>,
) -> anyhow::Result<()> {
    let song_performances: Vec<SongPerformanceJson> = parse_json_file("tracks").await?;

    let song_performances_by_set = get_song_performances_by_set(&song_performances);

    let song_performances_with_more_than_one_song = song_performances
        .iter()
        .filter(|song_performance| song_performance.songs.len() != 1)
        .collect::<Vec<_>>();
    println!("perf: {song_performances_with_more_than_one_song:#?}");

    unimplemented!();
    // insert_events(
    //     song_performances_by_set
    //         .iter()
    //         .flat_map(|(show_original_id, sets)| {
    //             sets.into_iter()
    //                 .flat_map(|(set_name, song_performances)| {
    //                     song_performances.into_iter().map(|song_performance| {
    //                         Event::InsertSongPerformance(SongPerformance {
    //                             id: song_performance.id,
    //                             set_id: sets[(show_original_id, set_name)],
    //                             song_id:
    //                         })
    //                     })
    //                 })
    //                 .collect::<Vec<_>>()
    //         })
    //         .map(|event| EventForInsertion::from(&event)),
    //     get_mutex_guard().await,
    //     db_pool,
    // )
    // .await;

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
