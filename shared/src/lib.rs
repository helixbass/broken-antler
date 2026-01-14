use std::env;

use tokio::net::TcpStream;
use uuid::Uuid;

mod event;
mod types;

pub use event::Event;
pub use types::{Set, SetName, Show, ShowRkyv, Song, SongPerformance, Venue};

pub async fn connect_to_db() -> TcpStream {
    let database_url = env::var("DATABASE_URL").unwrap();
    TcpStream::connect(&database_url).await.unwrap()
}

pub fn table_id() -> Uuid {
    Uuid::parse_str(&env::var("DATABASE_TABLE").unwrap()).unwrap()
}
