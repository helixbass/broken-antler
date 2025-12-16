use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod types;

pub use types::Venue;

pub async fn get_db_pool() -> anyhow::Result<Pool<Postgres>> {
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://broken_antler:password@localhost/broken_antler")
        .await?;

    Ok(db_pool)
}
