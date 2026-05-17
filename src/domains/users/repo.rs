use crate::domains::users::models::User;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: User) -> sqlx::Result<User, sqlx::Error>;
    async fn get_user(&self, id: &str) -> Option<User>;
    async fn update_user(&self, user: User) -> sqlx::Result<bool, sqlx::Error>;
    
}