use ::sauvignon::{schema, Schema};
use tracing::instrument;

mod database;
mod sauvignon;

pub use database::{get_database, Database, DatabaseCobbler, VenuesCobbler};
pub use shared::{Event, Set, Show, Song, Venue};

#[instrument(level = "trace")]
pub fn get_schema() -> Schema {
    schema! {
        types => [
            Venue => {
                fields => [
                    id => id_column()
                    name => string_column()
                    shows => has_many(
                        type => Show
                        foreign_key => venue_id
                    )
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
            Set => {
                fields => [
                    id => id_column()
                    show => belongs_to(
                        type => Show
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
            sets => {
                type => [Set!]!
                internal_dependencies => [
                    ids => id_column_list()
                ]
            }
        ]
    }
}
