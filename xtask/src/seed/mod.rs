use std::collections::HashMap;

use brunhilde::{client, tcp, AppendRows, Request, RowWithoutEventId};
use geoutils::Location;
use itertools::Itertools;
use serde::Deserialize;
use shared::{connect_to_db, table_id, Event, Set, SetName, Show, Song, SongPerformance, Venue};
use uuid::Uuid;

use crate::{
    get_show_original_ids, get_song_performances_by_set, parse_json_file, ShowJson,
    SongPerformanceJson,
};

pub async fn seed() -> anyhow::Result<()> {
    create_tables().await;
    let venue_slugs = seed_venues().await?;
    seed_songs().await?;
    seed_shows(&venue_slugs).await?;
    let sets = seed_sets().await?;
    seed_song_performances(&sets).await?;

    Ok(())
}

async fn seed_venues() -> anyhow::Result<HashMap<String, Uuid>> {
    let venues_json: Vec<VenueJson> = parse_json_file("venues").await?;

    let venue_slugs: HashMap<_, _> = venues_json
        .iter()
        .map(|venue| (venue.slug.clone(), venue.id))
        .collect();

    let venues: Vec<Venue> = venues_json.into_iter().map(Into::into).collect();
    // println!("venues: {venues:#?}");

    insert_events(venues.into_iter().map(|venue| Event::InsertVenue(venue))).await;

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

async fn seed_songs() -> anyhow::Result<()> {
    let songs: Vec<Song> = parse_json_file("songs").await?;
    // println!("songs: {songs:#?}");

    insert_events(songs.into_iter().map(|song| Event::InsertSong(song))).await;

    Ok(())
}

async fn seed_shows(venue_slugs: &HashMap<String, Uuid>) -> anyhow::Result<()> {
    let shows: Vec<Show> = parse_json_file::<Vec<ShowJson>>("shows")
        .await?
        .into_iter()
        .map(|show| show.process(venue_slugs))
        .collect();

    // println!("shows: {shows:#?}");

    insert_events(shows.into_iter().map(|show| Event::InsertShow(show))).await;

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

async fn seed_sets() -> anyhow::Result<HashMap<(u32, SetName), Uuid>> {
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

    insert_events(sets.into_iter().map(|set| Event::InsertSet(set))).await;

    Ok(sets_map)
}

async fn seed_song_performances(sets_map: &HashMap<(u32, SetName), Uuid>) -> anyhow::Result<()> {
    let song_performances: Vec<SongPerformanceJson> = parse_json_file("tracks").await?;

    let song_performances_by_set = get_song_performances_by_set(&song_performances);

    for events in &song_performances_by_set
        .iter()
        .flat_map(|(show_original_id, sets)| {
            sets.into_iter()
                .flat_map(|(set_name, song_performances)| {
                    song_performances
                        .into_iter()
                        .flat_map(|song_performance| {
                            song_performance
                                .songs
                                .iter()
                                .map(|song| {
                                    Event::InsertSongPerformance(SongPerformance {
                                        id: song_performance.id,
                                        set_id: sets_map[&(*show_original_id, *set_name)],
                                        song_id: song.id,
                                    })
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .chunks(1000)
    {
        insert_events(events).await;
    }

    Ok(())
}

async fn create_tables() {
    let tcp_stream = connect_to_db().await;
    client::request(tcp::Request::CreateTable(table_id()), tcp_stream)
        .await
        .as_create_table();
}

async fn insert_events(events: impl IntoIterator<Item = Event>) {
    let tcp_stream = connect_to_db().await;
    client::request(
        Request::from(AppendRows::new(
            table_id(),
            events
                .into_iter()
                .map(|event| RowWithoutEventId::from(&event))
                .collect(),
        ))
        .into(),
        tcp_stream,
    )
    .await
    .as_append_rows();
}
