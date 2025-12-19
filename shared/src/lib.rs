use std::sync::LazyLock;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::{Mutex, MutexGuard};

mod event;
mod types;

pub use event::Event;
pub use types::{Set, SetName, Show, Song, Venue};

pub async fn get_db_pool() -> anyhow::Result<Pool<Postgres>> {
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://broken_antler:password@localhost/broken_antler")
        .await?;

    Ok(db_pool)
}

static GUARD_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

pub async fn get_mutex_guard() -> MutexGuard<'static, ()> {
    GUARD_MUTEX.lock().await
}
