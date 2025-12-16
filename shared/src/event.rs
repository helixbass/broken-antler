use juriji::{from_json_str_with_id, to_serde_json_value_without_id, EventForInsertion, ReadEvent};

use crate::Venue;

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
