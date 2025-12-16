use geoutils::Location;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct Venue {
    pub id: Uuid,
    pub name: String,
    pub location: Location,
}
