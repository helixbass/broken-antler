use sauvignon::{DependencyType, DependencyValue};
use uuid::Uuid;

use crate::Database;

impl sauvignon::Database for Database {
    type Id = Uuid;

    fn get_column(
        &self,
        table_name: &str,
        column_name: &str,
        id: &Uuid,
        dependency_type: DependencyType,
    ) -> DependencyValue {
        unimplemented!()
    }

    fn get_column_list(
        &self,
        table_name: &str,
        column_name: &str,
        dependency_type: DependencyType,
    ) -> Vec<DependencyValue> {
        unimplemented!()
    }
}
