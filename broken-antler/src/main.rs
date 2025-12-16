use std::sync::Arc;

use sauvignon_axum::{axum, simple_app};
use tokio::net::TcpListener;

use broken_antler::{get_database, get_schema};
use shared::get_db_pool;

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
