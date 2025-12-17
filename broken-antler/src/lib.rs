use ::sauvignon::{schema, Schema};

mod database;
mod sauvignon;

pub use database::{get_database, Database, DatabaseCobbler, VenuesCobbler};
pub use shared::{Event, Show, Song, Venue};

pub fn get_schema() -> Schema {
    schema! {
        types => [
            Venue => {
                fields => [
                    id => id_column()
                    name => string_column()
                ]
            }
            Song => {
                fields => [
                    id => id_column()
                    title => string_column()
                ]
            }
            Show => {
                fields => [
                    id => id_column()
                    date => date_column()
                    venue => belongs_to(
                        type => Venue
                    )
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
            songs => {
                type => [Song!]!
                internal_dependencies => [
                    ids => id_column_list()
                ]
            }
            shows => {
                type => [Show!]!
                internal_dependencies => [
                    ids => id_column_list()
                ]
            }
        ]
    }
}
