use std::sync::LazyLock;

use async_trait::async_trait;
use sauvignon::{ColumnToken, ColumnTokens, DependencyType, DependencyValue, Id, WhereResolved};
use smol_str::ToSmolStr;
use tracing::instrument;

use crate::{Database, Show, Song, Venue};

#[async_trait]
impl sauvignon::Database for Database {
    async fn get_column(
        &self,
        _table_name: &str,
        _column_name: &str,
        _id: &Id,
        _id_column_name: &str,
        _dependency_type: DependencyType,
    ) -> DependencyValue {
        unreachable!()
    }

    async fn get_column_list(
        &self,
        _table_name: &str,
        _column_name: &str,
        _dependency_type: DependencyType,
        _wheres: &[WhereResolved],
    ) -> Vec<DependencyValue> {
        unreachable!()
    }

    // TODO: it looked like perhaps this can get called so frequently
    // that the tracing has some noticeable cost
    // #[instrument(level = "trace", skip(self))]
    fn get_column_sync(
        &self,
        table_name: &str,
        column_name: &str,
        id: &Id,
        id_column_name: &str,
        dependency_type: DependencyType,
    ) -> DependencyValue {
        assert_eq!(id_column_name, "id");
        let id = id.as_uuid();
        match table_name {
            "venues" => self
                .venue_by_id(id)
                .get_column(column_name, dependency_type),
            "songs" => self.song_by_id(id).get_column(column_name, dependency_type),
            "shows" => self.show_by_id(id).get_column(column_name, dependency_type),
            table_name => panic!("Unknown table name {table_name}"),
        }
    }

    // #[instrument(level = "trace", skip(self))]
    fn get_column_list_sync(
        &self,
        table_name: &str,
        column_name: &str,
        dependency_type: DependencyType,
        wheres: &[WhereResolved],
    ) -> Vec<DependencyValue> {
        match table_name {
            "venues" => {
                assert!(wheres.is_empty());
                self.venues
                    .iter()
                    // .filter(|venue| venue.matches_wheres(wheres))
                    .map(|venue| venue.get_column(column_name, dependency_type))
                    .collect()
            }
            "songs" => {
                assert!(wheres.is_empty());
                self.songs
                    .iter()
                    // .filter(|song| song.matches_wheres(wheres))
                    .map(|song| song.get_column(column_name, dependency_type))
                    .collect()
            }
            "shows" => match wheres.is_empty() {
                true => self
                    .shows
                    .iter()
                    .map(|show| show.get_column(column_name, dependency_type))
                    .collect(),
                false => {
                    assert!(wheres.len() == 1 && wheres[0].column_name == "venue_id");
                    self.shows_by_venue_id[wheres[0].value.as_id().as_uuid()]
                        .iter()
                        .map(|show_index| {
                            self.shows[*show_index].get_column(column_name, dependency_type)
                        })
                        .collect()
                }
            },
            table_name => panic!("Unknown table name {table_name}"),
        }
    }

    fn is_sync(&self) -> bool {
        true
    }

    fn column_tokens(&self) -> Option<&'static ColumnTokens> {
        Some({
            static COLUMN_TOKENS: LazyLock<ColumnTokens> = LazyLock::new(|| {
                [
                    (
                        "venues".to_smolstr(),
                        [
                            ("id".to_smolstr(), Database::VENUE_ID_COLUMN_TOKEN),
                            ("name".to_smolstr(), Database::VENUE_NAME_COLUMN_TOKEN),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                    (
                        "songs".to_smolstr(),
                        [
                            ("id".to_smolstr(), Database::SONG_ID_COLUMN_TOKEN),
                            ("title".to_smolstr(), Database::SONG_TITLE_COLUMN_TOKEN),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                    (
                        "shows".to_smolstr(),
                        [
                            ("id".to_smolstr(), Database::SHOW_ID_COLUMN_TOKEN),
                            ("date".to_smolstr(), Database::SHOW_DATE_COLUMN_TOKEN),
                            (
                                "venue_id".to_smolstr(),
                                Database::SHOW_VENUE_ID_COLUMN_TOKEN,
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                ]
                .into_iter()
                .collect()
            });
            &*COLUMN_TOKENS
        })
    }
}

impl Database {
    const VENUES_TABLE: u32 = 0;
    const SONGS_TABLE: u32 = 1;
    const SHOWS_TABLE: u32 = 2;

    const VENUE_ID_COLUMN: u32 = 0;
    const VENUE_NAME_COLUMN: u32 = 1;
    const SONG_ID_COLUMN: u32 = 2;
    const SONG_TITLE_COLUMN: u32 = 3;
    const SHOW_ID_COLUMN: u32 = 4;
    const SHOW_DATE_COLUMN: u32 = 5;
    const SHOW_VENUE_ID_COLUMN: u32 = 6;

    const VENUE_ID_COLUMN_TOKEN: ColumnToken = ColumnToken {
        table: Self::VENUES_TABLE,
        column: Self::VENUE_ID_COLUMN,
    };
    const VENUE_NAME_COLUMN_TOKEN: ColumnToken = ColumnToken {
        table: Self::VENUES_TABLE,
        column: Self::VENUE_NAME_COLUMN,
    };
    const SONG_ID_COLUMN_TOKEN: ColumnToken = ColumnToken {
        table: Self::SONGS_TABLE,
        column: Self::SONG_ID_COLUMN,
    };
    const SONG_TITLE_COLUMN_TOKEN: ColumnToken = ColumnToken {
        table: Self::SONGS_TABLE,
        column: Self::SONG_TITLE_COLUMN,
    };
    const SHOW_ID_COLUMN_TOKEN: ColumnToken = ColumnToken {
        table: Self::SHOWS_TABLE,
        column: Self::SHOW_ID_COLUMN,
    };
    const SHOW_DATE_COLUMN_TOKEN: ColumnToken = ColumnToken {
        table: Self::SHOWS_TABLE,
        column: Self::SHOW_DATE_COLUMN,
    };
    const SHOW_VENUE_ID_COLUMN_TOKEN: ColumnToken = ColumnToken {
        table: Self::SHOWS_TABLE,
        column: Self::SHOW_VENUE_ID_COLUMN,
    };
}

trait Row {
    fn get_column(&self, column_name: &str, dependency_type: DependencyType) -> DependencyValue;
}

impl Row for Venue {
    // #[instrument(level = "trace", skip(self))]
    fn get_column(&self, column_name: &str, dependency_type: DependencyType) -> DependencyValue {
        match column_name {
            "name" => {
                assert_eq!(dependency_type, DependencyType::String);
                DependencyValue::String(self.name.clone())
            }
            "id" => {
                assert!(matches!(
                    dependency_type,
                    DependencyType::Id | DependencyType::ListOfIds
                ));
                DependencyValue::Id(Id::Uuid(self.id))
            }
            _ => panic!("Unknown column: {column_name}"),
        }
    }
}

impl Row for Song {
    // #[instrument(level = "trace", skip(self))]
    fn get_column(&self, column_name: &str, dependency_type: DependencyType) -> DependencyValue {
        match column_name {
            "title" => {
                assert_eq!(dependency_type, DependencyType::String);
                DependencyValue::String(self.title.clone())
            }
            "id" => {
                assert!(matches!(
                    dependency_type,
                    DependencyType::Id | DependencyType::ListOfIds
                ));
                DependencyValue::Id(Id::Uuid(self.id))
            }
            _ => panic!("Unknown column: {column_name}"),
        }
    }
}

impl Row for Show {
    // #[instrument(level = "trace", skip(self))]
    fn get_column(&self, column_name: &str, dependency_type: DependencyType) -> DependencyValue {
        match column_name {
            "date" => {
                assert_eq!(dependency_type, DependencyType::Date);
                DependencyValue::Date(self.date.clone())
            }
            "id" => {
                assert!(matches!(
                    dependency_type,
                    DependencyType::Id | DependencyType::ListOfIds
                ));
                DependencyValue::Id(Id::Uuid(self.id))
            }
            "venue_id" => {
                assert!(matches!(
                    dependency_type,
                    DependencyType::Id | DependencyType::ListOfIds
                ));
                DependencyValue::Id(Id::Uuid(self.venue_id))
            }
            _ => panic!("Unknown column: {column_name}"),
        }
    }
}

#[allow(dead_code)]
trait MatchWheres {
    fn matches_wheres(&self, wheres: &[WhereResolved]) -> bool;
}

impl MatchWheres for Venue {
    #[instrument(level = "trace", skip(self))]
    fn matches_wheres(&self, wheres: &[WhereResolved]) -> bool {
        for _where in wheres {
            unimplemented!()
        }
        true
    }
}

impl MatchWheres for Song {
    #[instrument(level = "trace", skip(self))]
    fn matches_wheres(&self, wheres: &[WhereResolved]) -> bool {
        for _where in wheres {
            unimplemented!()
        }
        true
    }
}

impl MatchWheres for Show {
    // TODO: in samply profiling this tracing (I believe) looked
    // like it was actually costing a fair amount?
    // #[instrument(level = "trace", skip(self))]
    fn matches_wheres(&self, wheres: &[WhereResolved]) -> bool {
        for where_ in wheres {
            match &*where_.column_name {
                "venue_id" => {
                    if where_.value.as_id().as_uuid() != &self.venue_id {
                        return false;
                    }
                }
                _ => unimplemented!(),
            }
        }
        true
    }
}
