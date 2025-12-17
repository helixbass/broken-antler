use async_trait::async_trait;
use sauvignon::{DependencyType, DependencyValue, WhereResolved};
use uuid::Uuid;

use crate::{Database, Song, Venue};

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
        if !wheres.is_empty() {
            unimplemented!()
        }
        match table_name {
            "venues" => self
                .venues
                .values()
                .map(|venue| venue.get_column(column_name, dependency_type))
                .collect(),
            "songs" => self
                .songs
                .values()
                .map(|song| song.get_column(column_name, dependency_type))
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
