use sauvignon::{json_from_response, Database, Schema};

use broken_antler::{get_database, get_schema};
use shared::get_db_pool;

async fn run_request(request: &str, schema: &Schema, database: &dyn Database) {
    let response = schema.request(request, database).await;
    assert!(response.data.is_some());
    let _json = json_from_response(&response);
}

#[tokio::main]
async fn main() {
    let db_pool = get_db_pool().await.unwrap();
    let database = get_database(&db_pool).await;
    let schema = get_schema();

    run_request(
        r#"
            {
              venues {
                name
                shows {
                  date
                  venue {
                    name
                    id
                  }
                }
              }
            }
        "#,
        &schema,
        &database,
    )
    .await;
}
