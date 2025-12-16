use geoutils::Location;
use juriji::{insert_event, EventForInsertion};
use serde::Deserialize;
use shared::{get_db_pool, get_mutex_guard, Event, Venue};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::parse_json_file;

pub async fn seed() -> anyhow::Result<()> {
    let db_pool = get_db_pool().await.unwrap();
    seed_venues(&db_pool).await?;
    unimplemented!()
}

async fn seed_venues(db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let venues: Vec<Venue> = parse_json_file("venues").await?;
    println!("venues: {venues:#?}");

    for event in venues
        .into_iter()
        .map(|venue| Event::InsertVenue(venue))
        .map(|event| EventForInsertion::from(&event))
    {
        insert_event(event, get_mutex_guard().await, db_pool).await;
    }

    Ok(())
}

#[derive(Deserialize)]
struct VenueJson {
    pub id: Uuid,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl From<VenueJson> for Venue {
    fn from(value: VenueJson) -> Self {
        Self {
            id: value.id,
            name: value.name,
            location: Location::new(value.latitude, value.longitude),
        }
    }
}
