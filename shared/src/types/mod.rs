use geoutils::Location;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Venue {
    pub id: Uuid,
    pub name: String,
    pub location: Option<Location>,
}
