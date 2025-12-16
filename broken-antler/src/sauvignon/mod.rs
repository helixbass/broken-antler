use sauvignon::{DependencyType, DependencyValue};
use uuid::Uuid;

use crate::{Database, Venue};

impl sauvignon::Database for Database {
    type Id = Uuid;

    async fn get_column(
        &self,
        table_name: &str,
        column_name: &str,
        id: &Uuid,
        dependency_type: DependencyType,
    ) -> DependencyValue {
        match table_name {
            "venues" => self
                .venues
                .get(id)
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
    ) -> Vec<DependencyValue> {
        match table_name {
            "venues" => self
                .venues
                .values()
                .map(|venue| venue.get_column(column_name, dependency_type))
                .collect(),
            table_name => panic!("Unknown table name {table_name}"),
        }
    }
}

impl sauvignon::Row for Venue {
    fn get_column(&self, column_name: &str, dependency_type: DependencyType) -> DependencyValue {
        match column_name {
            "name" => {
                assert_eq!(dependency_type, DependencyType::String);
                DependencyValue::String(self.name.clone())
            }
            _ => panic!("Unknown column: {column_name}"),
        }
    }
}
