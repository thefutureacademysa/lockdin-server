use crate::domains::rooms::models::{Room, RoomRecord, SubjectCategory};
use crate::domains::rooms::repository::RoomRepository;
use sqlx::Error;

pub struct PostgresRoomRepository {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

#[async_trait::async_trait]
impl RoomRepository for PostgresRoomRepository {
    async fn get_all_rooms(
        &self,
        search: Option<String>,
        grade: Option<i32>,
        category: Option<SubjectCategory>,
    ) -> Result<Vec<RoomRecord>, Error> {
        let mut query = String::from("SELECT * FROM rooms WHERE 1=1");
        let mut param_count = 0;

        if grade.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND grade = ${}", param_count));
        }

        if category.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND category = ${}", param_count));
        }

        if search.is_some() {
            param_count += 1;
            query.push_str(&format!(
                " AND (
                    to_tsvector('english', name || ' ' || subject) @@ plainto_tsquery('english', ${p})
                    OR name ILIKE '%' || ${p} || '%'
                    OR subject ILIKE '%' || ${p} || '%'
                )",
                p = param_count
            ));
        }


        let mut sql_query = sqlx::query_as(&query);

        if let Some(g) = grade {
            sql_query = sql_query.bind(g);
        }

        if let Some(c) = category {
            sql_query = sql_query.bind(c.as_str());
        }

        if let Some(s) = search {
            sql_query = sql_query.bind(s);
        }

        sql_query.fetch_all(&self.pool).await
    }
}
