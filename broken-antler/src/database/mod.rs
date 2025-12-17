use derive_builder::Builder;
use indexmap::IndexMap;
use juriji::read_events;
use shared::Event;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{Song, Venue};

#[derive(Default)]
pub struct VenuesCobbler {
    pub assembling: IndexMap<Uuid, Venue>,
}

impl VenuesCobbler {
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

#[derive(Builder)]
pub struct Database {
    #[builder(setter(into))]
    pub venues: IndexMap<Uuid, Venue>,
    #[builder(setter(into))]
    pub songs: IndexMap<Uuid, Song>,
}

#[derive(Default)]
pub struct DatabaseCobbler {
    pub venues: VenuesCobbler,
    pub songs: SongsCobbler,
}

impl DatabaseCobbler {
    pub fn accept_next(&mut self, event: &Event) {
        self.venues.accept_next(event);
        self.songs.accept_next(event);
    }
}

impl From<DatabaseCobbler> for Database {
    fn from(value: DatabaseCobbler) -> Self {
        DatabaseBuilder::default()
            .venues(value.venues)
            .songs(value.songs)
            .build()
            .unwrap()
    }
}

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
