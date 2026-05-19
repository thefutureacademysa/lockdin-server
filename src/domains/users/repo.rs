use crate::domains::auth::error::AppError;
use crate::domains::users::models::User;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: User) -> sqlx::Result<User, AppError>;
    async fn get_user(&self, id: &str) -> sqlx::Result<Option<User>, sqlx::Error>;
    async fn get_user_by_phone(&self, id: &String) -> sqlx::Result<Option<User>, AppError>;
    async fn update_user(&self, user: User) -> sqlx::Result<bool, sqlx::Error>;
    
}