use ::sauvignon::{schema, Schema};

mod database;
mod sauvignon;

pub use database::{get_database, Database, DatabaseCobbler, VenuesCobbler};
pub use shared::{Event, Venue};

pub fn get_schema() -> Schema {
    schema! {
        types => [
            Venue => {
                fields => [
                    name => string_column()
                ]
            }
        ]
        query => [
            venues => {
                type => [Venue!]!
                internal_dependencies => [
                    ids => id_column_list()
                ]
            }
        ]
    }
}
