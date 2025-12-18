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
