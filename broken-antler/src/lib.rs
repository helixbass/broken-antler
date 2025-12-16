use std::collections::HashSet;

use geoutils::Location;
use indexmap::IndexMap;
use juriji::{CobbleAll, EventForInsertion, ReadEvent};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use squalid::_d;
use uuid::Uuid;

pub enum Event {
    InsertVenue(Venue),
}

// TODO: make a new strum-like macro to generate this
// eg #[variant_names]
pub trait EventVariantNames {
    const INSERT_VENUE: &'static str;
}

impl EventVariantNames for Event {
    const INSERT_VENUE: &'static str = "INSERT_VENUE";
}

impl From<&Event> for EventForInsertion {
    fn from(value: &Event) -> Self {
        use EventVariantNames;
        match value {
            Event::InsertVenue(venue) => EventForInsertion::new(
                Some(venue.id),
                // Event::NAMES::InsertVenue.to_owned(),
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

fn to_serde_json_value_without_id<TSerializable: Serialize>(
    value: &TSerializable,
) -> serde_json::Value {
    let mut value = serde_json::to_value(value).unwrap();
    let _ = value.as_object_mut().unwrap().remove("id").unwrap();
    value
}

#[derive(Deserialize, Serialize)]
pub struct Venue {
    pub id: Uuid,
    pub name: String,
    pub location: Location,
}

impl CobbleAll for Venue {
    fn relevant_event_types(&self) -> HashSet<String> {
        ["INSERT_VENUE".to_owned()].into_iter().collect()
    }

    fn cobble(events: &[ReadEvent]) -> Vec<Self> {
        let mut ret: IndexMap<Uuid, Self> = _d();
        for event in events.into_iter().map(|event| Event::from(event)) {
            match event {
                Event::InsertVenue(venue) => {
                    ret.insert(venue.id, venue);
                } // _ => unreachable!(),
            }
        }

        ret.into_values().collect()
    }
}

fn from_json_str_with_id<TTarget: DeserializeOwned>(json_str: &str, id: Uuid) -> TTarget {
    let mut value: serde_json::Value = serde_json::from_str(json_str).unwrap();
    let id_value = serde_json::to_value(id).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("id".to_owned(), id_value);
    serde_json::from_value(value).unwrap()
}
