use std::collections::HashMap;

use derive_builder::Builder;
use itertools::Itertools;
use juriji::read_events;
use shared::Event;
use sqlx::{Pool, Postgres};
use tracing::instrument;
use uuid::Uuid;

use crate::{Show, Song, Venue};

#[derive(Default)]
pub struct VenuesCobbler {
    pub assembling: Vec<Venue>,
}

impl VenuesCobbler {
    #[instrument(level = "trace", skip(self))]
    pub fn accept_next(&mut self, event: &Event) {
        match event {
            Event::InsertVenue(venue) => {
                self.assembling.push(venue.clone());
            }
            _ => {}
        }
    }
}

impl From<VenuesCobbler> for Vec<Venue> {
    fn from(value: VenuesCobbler) -> Self {
        value.assembling
    }
}

#[derive(Default)]
pub struct SongsCobbler {
    pub assembling: Vec<Song>,
}

impl SongsCobbler {
    #[instrument(level = "trace", skip(self))]
    pub fn accept_next(&mut self, event: &Event) {
        match event {
            Event::InsertSong(song) => {
                self.assembling.push(song.clone());
            }
            _ => {}
        }
    }
}

impl From<SongsCobbler> for Vec<Song> {
    fn from(value: SongsCobbler) -> Self {
        value.assembling
    }
}

#[derive(Default)]
pub struct ShowsCobbler {
    pub assembling: Vec<Show>,
}

impl ShowsCobbler {
    #[instrument(level = "trace", skip(self))]
    pub fn accept_next(&mut self, event: &Event) {
        match event {
            Event::InsertShow(show) => {
                self.assembling.push(show.clone());
            }
            _ => {}
        }
    }
}

impl From<ShowsCobbler> for Vec<Show> {
    fn from(value: ShowsCobbler) -> Self {
        value.assembling
    }
}

#[derive(Builder)]
pub struct Database {
    #[builder(setter(skip), default = "self.default_venues_by_id()")]
    pub venues_by_id: HashMap<Uuid, usize>,
    #[builder(setter(skip), default = "self.default_songs_by_id()")]
    pub songs_by_id: HashMap<Uuid, usize>,
    #[builder(setter(skip), default = "self.default_shows_by_id()")]
    pub shows_by_id: HashMap<Uuid, usize>,
    #[builder(setter(skip), default = "self.default_shows_by_venue_id()")]
    pub shows_by_venue_id: HashMap<Uuid, Vec<usize>>,
    #[builder(setter(into))]
    pub venues: Vec<Venue>,
    #[builder(setter(into))]
    pub songs: Vec<Song>,
    #[builder(setter(into))]
    pub shows: Vec<Show>,
}

impl DatabaseBuilder {
    fn default_venues_by_id(&self) -> HashMap<Uuid, usize> {
        self.venues
            .as_ref()
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(index, venue)| (venue.id, index))
            .collect()
    }

    fn default_songs_by_id(&self) -> HashMap<Uuid, usize> {
        self.songs
            .as_ref()
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(index, song)| (song.id, index))
            .collect()
    }

    fn default_shows_by_id(&self) -> HashMap<Uuid, usize> {
        self.shows
            .as_ref()
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(index, show)| (show.id, index))
            .collect()
    }

    fn default_shows_by_venue_id(&self) -> HashMap<Uuid, Vec<usize>> {
        self.shows
            .as_ref()
            .unwrap()
            .into_iter()
            .enumerate()
            .into_group_map_by(|(_index, show)| show.venue_id)
            .into_iter()
            .map(|(venue_id, shows_and_indexes)| {
                (
                    venue_id,
                    shows_and_indexes
                        .into_iter()
                        .map(|(index, _)| index)
                        .collect::<Vec<_>>(),
                )
            })
            .collect()
    }
}

impl Database {
    pub fn venue_by_id(&self, id: &Uuid) -> &Venue {
        &self.venues[self.venues_by_id[id]]
    }

    pub fn song_by_id(&self, id: &Uuid) -> &Song {
        &self.songs[self.songs_by_id[id]]
    }

    pub fn show_by_id(&self, id: &Uuid) -> &Show {
        &self.shows[self.shows_by_id[id]]
    }
}

#[derive(Default)]
pub struct DatabaseCobbler {
    pub venues: VenuesCobbler,
    pub songs: SongsCobbler,
    pub shows: ShowsCobbler,
}

impl DatabaseCobbler {
    #[instrument(level = "trace", skip(self))]
    pub fn accept_next(&mut self, event: &Event) {
        self.venues.accept_next(event);
        self.songs.accept_next(event);
        self.shows.accept_next(event);
    }
}

impl From<DatabaseCobbler> for Database {
    fn from(value: DatabaseCobbler) -> Self {
        DatabaseBuilder::default()
            .venues(value.venues)
            .songs(value.songs)
            .shows(value.shows)
            .build()
            .unwrap()
    }
}

#[instrument(level = "trace", skip(db_pool))]
pub async fn get_database(db_pool: &Pool<Postgres>) -> Database {
    let mut cobbler = DatabaseCobbler::default();
    read_events(None, db_pool)
        .await
        .into_iter()
        .map(|event| Event::from(&event))
        .for_each(|event| {
            cobbler.accept_next(&event);
        });
    cobbler.into()
}
