use shared::{get_db_pool, Venue};
use sqlx::{Pool, Postgres};

use crate::parse_json_file;

pub async fn seed() -> anyhow::Result<()> {
    let db_pool = get_db_pool().await.unwrap();
    seed_venues(&db_pool).await?;
    unimplemented!()
}

async fn seed_venues(db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let venues: Vec<Venue> = parse_json_file("venues").await?;
    println!("venues: {venues:#?}");

    unimplemented!()
}
