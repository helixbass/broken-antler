use brunhilde::{Row, RowWithoutEventId};
use rkyv::rancor;
use smol_str::ToSmolStr;

use crate::{Set, Show, ShowRkyv, Song, SongPerformance, Venue};

#[derive(Debug)]
pub enum Event {
    InsertVenue(Venue),
    InsertSong(Song),
    InsertShow(Show),
    InsertSet(Set),
    InsertSongPerformance(SongPerformance),
}

// TODO: make a new strum-like macro to generate this
// eg #[variant_names]
impl Event {
    const INSERT_VENUE: &'static str = "INSERT_VENUE";
    const INSERT_SONG: &'static str = "INSERT_SONG";
    const INSERT_SHOW: &'static str = "INSERT_SHOW";
    const INSERT_SET: &'static str = "INSERT_SET";
    const INSERT_SONG_PERFORMANCE: &'static str = "INSERT_SONG_PERFORMANCE";
}

impl From<&Event> for RowWithoutEventId {
    fn from(value: &Event) -> Self {
        match value {
            Event::InsertVenue(venue) => RowWithoutEventId::new(
                Some(venue.id),
                Event::INSERT_VENUE.to_smolstr(),
                rkyv::to_bytes::<rancor::Error>(venue).unwrap().into_vec(),
            ),
            Event::InsertSong(song) => RowWithoutEventId::new(
                Some(song.id),
                Event::INSERT_SONG.to_smolstr(),
                rkyv::to_bytes::<rancor::Error>(song).unwrap().into_vec(),
            ),
            Event::InsertShow(show) => RowWithoutEventId::new(
                Some(show.id),
                Event::INSERT_SHOW.to_smolstr(),
                rkyv::to_bytes::<rancor::Error>(&ShowRkyv::from(show))
                    .unwrap()
                    .into_vec(),
            ),
            Event::InsertSet(set) => RowWithoutEventId::new(
                Some(set.id),
                Event::INSERT_SET.to_smolstr(),
                rkyv::to_bytes::<rancor::Error>(set).unwrap().into_vec(),
            ),
            Event::InsertSongPerformance(song_performance) => RowWithoutEventId::new(
                Some(song_performance.id),
                Event::INSERT_SONG_PERFORMANCE.to_smolstr(),
                rkyv::to_bytes::<rancor::Error>(song_performance)
                    .unwrap()
                    .into_vec(),
            ),
        }
    }
}

impl From<&Row> for Event {
    fn from(value: &Row) -> Self {
        match &*value.type_ {
            Event::INSERT_VENUE => {
                Self::InsertVenue(rkyv::from_bytes::<_, rancor::Error>(&value.payload).unwrap())
            }
            Event::INSERT_SONG => {
                Self::InsertSong(rkyv::from_bytes::<_, rancor::Error>(&value.payload).unwrap())
            }
            Event::INSERT_SHOW => Self::InsertShow(
                rkyv::from_bytes::<ShowRkyv, rancor::Error>(&value.payload)
                    .unwrap()
                    .into(),
            ),
            Event::INSERT_SET => {
                Self::InsertSet(rkyv::from_bytes::<_, rancor::Error>(&value.payload).unwrap())
            }
            Event::INSERT_SONG_PERFORMANCE => Self::InsertSongPerformance(
                rkyv::from_bytes::<_, rancor::Error>(&value.payload).unwrap(),
            ),
            type_ => panic!("Unknown event type: {type_}"),
        }
    }
}
