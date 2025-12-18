use broken_antler::{get_database, get_schema};
use sauvignon::json_from_response;
use serde_json_path::{JsonPath, NodeList};

use shared::get_db_pool;

async fn request_test(request: &str, expected: impl FnOnce(&serde_json::Value)) {
    let db_pool = get_db_pool().await.unwrap();
    let database = get_database(&db_pool).await;
    let schema = get_schema();
    let response = schema.request(request, &database).await;
    let json = json_from_response(&response);
    let json: serde_json::Value = serde_json::from_str(&json).unwrap();
    expected(&json);
}

#[tokio::test]
async fn test_venues() {
    request_test(
        r#"
            {
              venues {
                id
                name
                shows {
                  date
                }
              }
            }
        "#,
        |response| {
            assert_eq!(_q("$.data.venues.*", response).len(), 764);
            assert_eq!(
                _q("$.data.venues[0].id", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "e9c1417c-3b09-48ef-8d7e-d5083d86f00b"
            );
            assert_eq!(
                _q("$.data.venues[0].name", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "13x13 Club"
            );
            assert_eq!(_q("$.data.venues[1].shows.*", response).len(), 3);
            assert_eq!(
                _q("$.data.venues[1].shows[0].date", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "2010-10-10"
            );
        },
    )
    .await;
}

#[tokio::test]
async fn test_songs() {
    request_test(
        r#"
            {
              songs {
                id
                title
              }
            }
        "#,
        |response| {
            assert_eq!(_q("$.data.songs.*", response).len(), 973);
            assert_eq!(
                _q("$.data.songs[0].id", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "9513527a-6047-485a-9a63-643618e98509"
            );
            assert_eq!(
                _q("$.data.songs[0].title", response)
                    .exactly_one()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "(I Can’t Get No) Satisfaction"
            );
        },
    )
    .await;
}

fn _q<'a>(query: &str, response: &'a serde_json::Value) -> NodeList<'a> {
    let path = JsonPath::parse(query).unwrap();
    path.query(response)
}
