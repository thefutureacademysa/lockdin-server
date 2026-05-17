use sqlx::Error;
use crate::domains::users::models::User;
use crate::domains::users::repo::UserRepository;

pub struct UserPostgresRepo {
    pub pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl UserRepository for UserPostgresRepo {
    async fn create_user(&self, user: User) -> sqlx::Result<User, Error> {
        todo!()
    }

    async fn get_user(&self, id: &str) -> Option<User> {
        todo!()
    }

    async fn update_user(&self, user: User) -> sqlx::Result<bool, Error> {
        todo!()
    }
}