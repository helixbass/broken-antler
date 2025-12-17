use derive_builder::Builder;
use indexmap::IndexMap;
use juriji::read_events;
use shared::Event;
use sqlx::{Pool, Postgres};
use tracing::instrument;
use uuid::Uuid;

use crate::{Show, Song, Venue};

#[derive(Default)]
pub struct VenuesCobbler {
    pub assembling: IndexMap<Uuid, Venue>,
}

impl VenuesCobbler {
    #[instrument(level = "trace", skip(self))]
    pub fn accept_next(&mut self, event: &Event) {
        match event {
            Event::InsertVenue(venue) => {
                self.assembling.insert(venue.id, venue.clone());
            }
            _ => {}
        }
    }
}

impl From<VenuesCobbler> for IndexMap<Uuid, Venue> {
    fn from(value: VenuesCobbler) -> Self {
        value.assembling
    }
}

#[derive(Default)]
pub struct SongsCobbler {
    pub assembling: IndexMap<Uuid, Song>,
}

impl SongsCobbler {
    #[instrument(level = "trace", skip(self))]
    pub fn accept_next(&mut self, event: &Event) {
        match event {
            Event::InsertSong(song) => {
                self.assembling.insert(song.id, song.clone());
            }
            _ => {}
        }
    }
}

impl From<SongsCobbler> for IndexMap<Uuid, Song> {
    fn from(value: SongsCobbler) -> Self {
        value.assembling
    }
}

#[derive(Default)]
pub struct ShowsCobbler {
    pub assembling: IndexMap<Uuid, Show>,
}

impl ShowsCobbler {
    #[instrument(level = "trace", skip(self))]
    pub fn accept_next(&mut self, event: &Event) {
        match event {
            Event::InsertShow(show) => {
                self.assembling.insert(show.id, show.clone());
            }
            _ => {}
        }
    }
}

impl From<ShowsCobbler> for IndexMap<Uuid, Show> {
    fn from(value: ShowsCobbler) -> Self {
        value.assembling
    }
}

#[derive(Builder)]
pub struct Database {
    #[builder(setter(into))]
    pub venues: IndexMap<Uuid, Venue>,
    #[builder(setter(into))]
    pub songs: IndexMap<Uuid, Song>,
    #[builder(setter(into))]
    pub shows: IndexMap<Uuid, Show>,
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
