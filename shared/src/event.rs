use juriji::{from_json_str_with_id, to_serde_json_value_without_id, EventForInsertion, ReadEvent};

use crate::{Set, Show, Song, Venue};

#[derive(Debug)]
pub enum Event {
    InsertVenue(Venue),
    InsertSong(Song),
    InsertShow(Show),
    InsertSet(Set),
}

// TODO: make a new strum-like macro to generate this
// eg #[variant_names]
impl Event {
    const INSERT_VENUE: &'static str = "INSERT_VENUE";
    const INSERT_SONG: &'static str = "INSERT_SONG";
    const INSERT_SHOW: &'static str = "INSERT_SHOW";
    const INSERT_SET: &'static str = "INSERT_SET";
}

impl From<&Event> for EventForInsertion {
    fn from(value: &Event) -> Self {
        match value {
            Event::InsertVenue(venue) => EventForInsertion::new(
                Some(venue.id),
                Event::INSERT_VENUE.to_owned(),
                to_serde_json_value_without_id(venue),
            ),
            Event::InsertSong(song) => EventForInsertion::new(
                Some(song.id),
                Event::INSERT_SONG.to_owned(),
                to_serde_json_value_without_id(song),
            ),
            Event::InsertShow(show) => EventForInsertion::new(
                Some(show.id),
                Event::INSERT_SHOW.to_owned(),
                to_serde_json_value_without_id(show),
            ),
            Event::InsertSet(set) => EventForInsertion::new(
                Some(set.id),
                Event::INSERT_SET.to_owned(),
                to_serde_json_value_without_id(set),
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
            Event::INSERT_SONG => {
                Self::InsertSong(from_json_str_with_id(&value.payload, value.id.unwrap()))
            }
            Event::INSERT_SHOW => {
                Self::InsertShow(from_json_str_with_id(&value.payload, value.id.unwrap()))
            }
            Event::INSERT_SET => {
                Self::InsertSet(from_json_str_with_id(&value.payload, value.id.unwrap()))
            }
            type_ => panic!("Unknown event type: {type_}"),
        }
    }
}
