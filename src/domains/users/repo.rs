use crate::domains::auth::error::AppError;
use crate::domains::users::models::User;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: User) -> sqlx::Result<User, AppError>;
    async fn get_user(&self, id: &str) -> sqlx::Result<Option<User>, sqlx::Error>;
    async fn get_user_by_email(&self, email: &String) -> sqlx::Result<Option<User>, AppError>;
    async fn verify_user(&self, email: &String) -> sqlx::Result<u64, sqlx::Error>; 
}