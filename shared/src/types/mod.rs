use chrono::NaiveDate;
use geoutils::Location;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Venue {
    pub id: Uuid,
    pub name: SmolStr,
    pub location: Option<Location>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Song {
    pub id: Uuid,
    pub title: SmolStr,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Show {
    pub id: Uuid,
    pub date: NaiveDate,
    pub venue_id: Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Set {
    pub id: Uuid,
    pub show_id: Uuid,
    pub set_name: SetName,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
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
