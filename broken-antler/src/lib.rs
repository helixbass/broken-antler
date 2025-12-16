use derive_builder::Builder;
use geoutils::Location;
use indexmap::IndexMap;
use juriji::{from_json_str_with_id, to_serde_json_value_without_id, EventForInsertion, ReadEvent};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub enum Event {
    InsertVenue(Venue),
}

// TODO: make a new strum-like macro to generate this
// eg #[variant_names]
impl Event {
    const INSERT_VENUE: &'static str = "INSERT_VENUE";
}

impl From<&Event> for EventForInsertion {
    fn from(value: &Event) -> Self {
        match value {
            Event::InsertVenue(venue) => EventForInsertion::new(
                Some(venue.id),
                Event::INSERT_VENUE.to_owned(),
                to_serde_json_value_without_id(venue),
            ),
        }
    }
}

impl From<&ReadEvent> for Event {
    fn from(value: &ReadEvent) -> Self {
        match &*value.type_ {
            Event::INSERT_VENUE => {
                Self::InsertVenue(from_json_str_with_id(&value.payload, value.id.unwrap()))
            }
            type_ => panic!("Unknown event type: {type_}"),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Venue {
    pub id: Uuid,
    pub name: String,
    pub location: Location,
}

#[derive(Default)]
pub struct VenuesCobbler {
    pub assembling: IndexMap<Uuid, Venue>,
}

impl VenuesCobbler {
    pub fn accept_next(&mut self, event: &Event) {
        match event {
            Event::InsertVenue(venue) => {
                self.assembling.insert(venue.id, venue.clone());
            } // _ => unreachable!(),
        }
    }
}

impl From<VenuesCobbler> for IndexMap<Uuid, Venue> {
    fn from(value: VenuesCobbler) -> Self {
        value.assembling
    }
}

#[derive(Builder)]
pub struct Database {
    #[builder(setter(into))]
    pub venues: IndexMap<Uuid, Venue>,
}

pub struct DatabaseCobbler {
    pub venues: VenuesCobbler,
}

impl DatabaseCobbler {
    pub fn accept_next(&mut self, event: &Event) {
        self.venues.accept_next(event);
    }
}

impl From<DatabaseCobbler> for Database {
    fn from(value: DatabaseCobbler) -> Self {
        DatabaseBuilder::default()
            .venues(value.venues)
            .build()
            .unwrap()
    }
}
