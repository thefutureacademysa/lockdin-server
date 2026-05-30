use sqlx::Error;
use crate::domains::rooms::models::{Room, SubjectCategory};
use crate::domains::rooms::repository::RoomRepository;

pub struct PostgresRoomRepository {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

#[async_trait::async_trait]
impl RoomRepository for PostgresRoomRepository {
    async fn get_all_rooms(&self, grade: Option<i32>, category: Option<SubjectCategory>) -> sqlx::Result<Vec<Room>, Error> {
        let mut query = String::from("SELECT * FROM rooms WHERE 1=1");

        if grade.is_some() {
            query.push_str(" AND grade = $1");
        }

        if category.is_some() {
            if grade.is_some() {
                query.push_str(" AND category = $2");
            } else {
                query.push_str(" AND category = $1");
            }
        }

        let mut sql_query = sqlx::query_as(&query);

        if let Some(g) = grade {
            sql_query = sql_query.bind(g);
        }

        if let Some(c) = category {
            log::info!("Category: {}", c.as_str());
            sql_query = sql_query.bind(c.as_str())
        }

        sql_query.fetch_all(&self.pool).await
    }
}