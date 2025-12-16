use std::sync::Arc;

use sauvignon_axum::{axum, simple_app};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::net::TcpListener;

use broken_antler::{get_database, get_schema};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let schema = get_schema();
    let db_pool = get_db_pool().await?;
    let database = get_database(&db_pool).await;

    axum::serve(
        TcpListener::bind("0.0.0.0:3001").await?,
        simple_app(Arc::new(schema), Arc::new(database)),
    )
    .await?;

    Ok(())
}

async fn get_db_pool() -> anyhow::Result<Pool<Postgres>> {
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://broken_antler:password@localhost/broken_antler")
        .await?;

    Ok(db_pool)
}
