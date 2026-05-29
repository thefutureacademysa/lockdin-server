use crate::domains::auth::models::RefreshToken;

#[async_trait::async_trait]
pub trait RefreshTokenRepo {
    async fn store_refresh_token(&self, rt: &RefreshToken) -> sqlx::Result<RefreshToken, sqlx::Error>;
    async fn get_refresh_token(&self, rt: &String) -> sqlx::Result<Option<RefreshToken>, sqlx::Error>;
    async fn revoke_refresh_token(&self, rt: &String) -> sqlx::Result<u64, sqlx::Error>;
    async fn invalidate_refresh_token(&self, rt: &String) -> sqlx::Result<u64, sqlx::Error>;
}