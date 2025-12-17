use async_trait::async_trait;
use sauvignon::{DependencyType, DependencyValue, WhereResolved};
use uuid::Uuid;

use crate::{Database, Show, Song, Venue};

#[async_trait]
impl sauvignon::Database for Database {
    async fn get_column(
        &self,
        table_name: &str,
        column_name: &str,
        id: &str,
        id_column_name: &str,
        dependency_type: DependencyType,
    ) -> DependencyValue {
        assert_eq!(id_column_name, "id");
        let id = Uuid::parse_str(id).unwrap();
        match table_name {
            "venues" => self
                .venues
                .get(&id)
                .unwrap()
                .get_column(column_name, dependency_type),
            "songs" => self
                .songs
                .get(&id)
                .unwrap()
                .get_column(column_name, dependency_type),
            "shows" => self
                .shows
                .get(&id)
                .unwrap()
                .get_column(column_name, dependency_type),
            table_name => panic!("Unknown table name {table_name}"),
        }
    }

    async fn get_column_list(
        &self,
        table_name: &str,
        column_name: &str,
        dependency_type: DependencyType,
        wheres: &[WhereResolved],
    ) -> Vec<DependencyValue> {
        match table_name {
            "venues" => self
                .venues
                .values()
                .filter(|venue| venue.matches_wheres(wheres))
                .map(|venue| venue.get_column(column_name, dependency_type))
                .collect(),
            "songs" => self
                .songs
                .values()
                .filter(|song| song.matches_wheres(wheres))
                .map(|song| song.get_column(column_name, dependency_type))
                .collect(),
            "shows" => self
                .shows
                .values()
                .filter(|show| show.matches_wheres(wheres))
                .map(|show| show.get_column(column_name, dependency_type))
                .collect(),
            table_name => panic!("Unknown table name {table_name}"),
        }
    }
}

trait Row {
    fn get_column(&self, column_name: &str, dependency_type: DependencyType) -> DependencyValue;
}

impl Row for Venue {
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
                DependencyValue::Id(self.id.to_string())
            }
            _ => panic!("Unknown column: {column_name}"),
        }
    }
}

impl Row for Song {
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
                DependencyValue::Id(self.id.to_string())
            }
            _ => panic!("Unknown column: {column_name}"),
        }
    }
}

impl Row for Show {
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
                DependencyValue::Id(self.id.to_string())
            }
            "venue_id" => {
                assert!(matches!(
                    dependency_type,
                    DependencyType::Id | DependencyType::ListOfIds
                ));
                DependencyValue::Id(self.venue_id.to_string())
            }
            _ => panic!("Unknown column: {column_name}"),
        }
    }
}

trait MatchWheres {
    fn matches_wheres(&self, wheres: &[WhereResolved]) -> bool;
}

impl MatchWheres for Venue {
    fn matches_wheres(&self, wheres: &[WhereResolved]) -> bool {
        for _where in wheres {
            unimplemented!()
        }
        true
    }
}

impl MatchWheres for Song {
    fn matches_wheres(&self, wheres: &[WhereResolved]) -> bool {
        for _where in wheres {
            unimplemented!()
        }
        true
    }
}

impl MatchWheres for Show {
    fn matches_wheres(&self, wheres: &[WhereResolved]) -> bool {
        for where_ in wheres {
            match &*where_.column_name {
                "venue_id" => {
                    assert!(matches!(where_.value, DependencyValue::Id(_)));
                    if where_.value.as_id() != &self.venue_id.to_string() {
                        return false;
                    }
                }
                _ => unimplemented!(),
            }
        }
        true
    }
}
