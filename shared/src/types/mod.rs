use chrono::{Datelike, NaiveDate};
use geoutils::Location;
use rkyv::{Archive, Deserialize, Serialize};
use smol_str::SmolStr;
use uuid::Uuid;

#[derive(Clone, Debug, Archive, Deserialize, Serialize, serde::Deserialize)]
pub struct Venue {
    pub id: Uuid,
    pub name: SmolStr,
    pub location: Option<Location>,
}

#[derive(Clone, Debug, Archive, Deserialize, Serialize, serde::Deserialize)]
pub struct Song {
    pub id: Uuid,
    pub title: SmolStr,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Show {
    pub id: Uuid,
    pub date: NaiveDate,
    pub venue_id: Uuid,
}

#[derive(Clone, Debug, Archive, Deserialize, Serialize)]
pub struct ShowRkyv {
    pub id: Uuid,
    pub date: NaiveDateRkyv,
    pub venue_id: Uuid,
}

impl From<Show> for ShowRkyv {
    fn from(value: Show) -> Self {
        Self {
            id: value.id,
            date: value.date.into(),
            venue_id: value.venue_id,
        }
    }
}

impl<'a> From<&'a Show> for ShowRkyv {
    fn from(value: &'a Show) -> Self {
        Self {
            id: value.id,
            date: value.date.into(),
            venue_id: value.venue_id,
        }
    }
}

impl From<ShowRkyv> for Show {
    fn from(value: ShowRkyv) -> Self {
        Self {
            id: value.id,
            date: value.date.into(),
            venue_id: value.venue_id,
        }
    }
}

#[derive(Copy, Clone, Debug, Archive, Deserialize, Serialize)]
pub struct NaiveDateRkyv {
    pub month: u32,
    pub year: i32,
    pub day: u32,
}

impl From<NaiveDate> for NaiveDateRkyv {
    fn from(value: NaiveDate) -> Self {
        Self {
            month: value.month(),
            year: value.year(),
            day: value.day(),
        }
    }
}

impl From<NaiveDateRkyv> for NaiveDate {
    fn from(value: NaiveDateRkyv) -> Self {
        Self::from_ymd_opt(value.year, value.month, value.day).unwrap()
    }
}

#[derive(Clone, Debug, Archive, Deserialize, Serialize, serde::Deserialize, serde::Serialize)]
pub struct Set {
    pub id: Uuid,
    pub show_id: Uuid,
    pub set_name: SetName,
}

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Archive,
    Deserialize,
    Serialize,
    serde::Deserialize,
    serde::Serialize,
    Hash,
    strum::Display,
)]
pub enum SetName {
    #[serde(alias = "Set 1")]
    Set1,
    #[serde(alias = "Set 2")]
    Set2,
    #[serde(alias = "Set 3")]
    Set3,
    #[serde(alias = "Set 4")]
    Set4,
    Encore,
    #[serde(alias = "Encore 2")]
    Encore2,
    #[serde(alias = "Encore 3")]
    Encore3,
    #[serde(alias = "Pre-Show")]
    PreShow,
    Soundcheck,
    #[serde(alias = "Unknown Set")]
    Unknown,
}

#[derive(Clone, Debug, Archive, Deserialize, Serialize, serde::Deserialize)]
pub struct SongPerformance {
    pub id: Uuid,
    pub set_id: Uuid,
    pub song_id: Uuid,
}
