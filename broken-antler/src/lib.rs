use ::sauvignon::{
    schema, CarverOrPopulator, ExternalDependencyValues, InternalDependencyValues, PopulatorList,
    PopulatorListInterface, Schema, UnionOrInterfaceTypePopulatorList,
};
use smol_str::SmolStr;
use tracing::instrument;

mod database;
mod sauvignon;

pub use database::{get_database, Database, DatabaseCobbler, VenuesCobbler};
pub use shared::{Event, Set, Show, Song, Venue};

pub struct SearchResultsTypePopulator {}

impl SearchResultsTypePopulator {
    pub fn new() -> Self {
        Self {}
    }
}

impl UnionOrInterfaceTypePopulatorList for SearchResultsTypePopulator {
    fn populate(
        &self,
        _external_dependencies: &ExternalDependencyValues,
        internal_dependencies: &InternalDependencyValues,
    ) -> Vec<SmolStr> {
        internal_dependencies
            .get("search_results")
            .unwrap()
            .as_list()
            .into_iter()
            .map(|search_result| search_result.as_map()["type"].as_string().clone())
            .collect()
    }
}

pub struct SearchResultsPopulator {}

impl SearchResultsPopulator {
    pub fn new() -> Self {
        Self {}
    }
}

impl PopulatorListInterface for SearchResultsPopulator {
    fn populate(
        &self,
        _external_dependencies: &ExternalDependencyValues,
        internal_dependencies: &InternalDependencyValues,
    ) -> Vec<ExternalDependencyValues> {
        internal_dependencies
            .get("search_results")
            .unwrap()
            .as_list()
            .into_iter()
            .map(|search_result| {
                [("id".into(), search_result.as_map()["id"].clone())]
                    .into_iter()
                    .collect()
            })
            .collect()
    }
}

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
            search => {
                params => [
                    query => String!
                ]
                type => [SearchResult!]!
                internal_dependencies => [
                    search_results => custom_sync {
                    }
                ]
                populator => custom {
                    CarverOrPopulator::UnionOrInterfaceTypePopulatorList(
                        Box::new(SearchResultsTypePopulator::new()),
                        PopulatorList::Dyn(Box::new(SearchResultsPopulator::new())),
                    )
                }
            }
        ]
        unions => [
            SearchResult => [Show, Song, Venue],
        ]
    }
}
